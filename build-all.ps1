<#
.SYNOPSIS
    Build all three bridge targets:
      1. WASM + TypeScript  (pkg/ + dist/)
      2. C native library   (native/target/.../release/)
      3. Java/JNI library   (same .dll/.so with JNI symbols)

.PARAMETER Debug
    Build C/JNI in debug profile instead of release.

.PARAMETER SkipWasm
    Skip the WASM/TypeScript build.

.PARAMETER SkipC
    Skip the C native build.

.PARAMETER SkipJni
    Skip the Java/JNI build.

.PARAMETER Target
    Override the Rust cross-compile target triple.
    Defaults to auto-detected host triple.

.EXAMPLE
    .\build-all.ps1                    # build everything, release
    .\build-all.ps1 -SkipWasm          # only C + JNI
    .\build-all.ps1 -SkipC -SkipJni   # only WASM/TS
    .\build-all.ps1 -Debug             # C/JNI in debug profile
    .\build-all.ps1 -Target aarch64-unknown-linux-gnu
#>
param(
    [switch]$Debug,
    [switch]$SkipWasm,
    [switch]$SkipC,
    [switch]$SkipJni,
    [string]$Target = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# ── helpers ───────────────────────────────────────────────────────────────────

function Write-Step([string]$msg) {
    Write-Host ""
    Write-Host "===========================================" -ForegroundColor DarkCyan
    Write-Host "  $msg" -ForegroundColor Cyan
    Write-Host "===========================================" -ForegroundColor DarkCyan
}

function Write-Ok([string]$msg)   { Write-Host "  [OK]   $msg" -ForegroundColor Green }
function Write-Info([string]$msg) { Write-Host "  [ ]   $msg" -ForegroundColor Gray  }
function Write-Err([string]$msg)  { Write-Host "  [ERR] $msg" -ForegroundColor Red   }

function Invoke-Cmd([string]$cmd, [string]$workDir = $PSScriptRoot) {
    Write-Info $cmd
    Push-Location $workDir
    try {
        # Native tools (cargo, wasm-pack) write progress/warnings to stderr. Under
        # $ErrorActionPreference="Stop" that first stderr line is promoted to a
        # terminating error before we ever see the exit code, so the build aborts
        # on a harmless warning. Drop to Continue for the call and judge success
        # by $LASTEXITCODE only.
        $prev = $ErrorActionPreference
        $ErrorActionPreference = 'Continue'
        try {
            Invoke-Expression $cmd
        } finally {
            $ErrorActionPreference = $prev
        }
        if ($LASTEXITCODE -ne 0) { throw "Command failed (exit $LASTEXITCODE): $cmd" }
    } finally {
        Pop-Location
    }
}

function Get-HostTarget {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture
    $isWin = [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform(
                 [System.Runtime.InteropServices.OSPlatform]::Windows)
    $isMac = [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform(
                 [System.Runtime.InteropServices.OSPlatform]::OSX)

    $archStr = switch ($arch.ToString().ToLower()) {
        "x64"   { "x86_64" }
        "arm64" { "aarch64" }
        default { throw "Unsupported arch: $arch" }
    }

    if ($isWin) { return "$archStr-pc-windows-msvc" }
    if ($isMac) { return "$archStr-apple-darwin" }
    return "$archStr-unknown-linux-gnu"
}

function Get-LibName([string]$rustTarget) {
    if ($rustTarget -like "*windows*") { return "whatsapp_bridge.dll" }
    if ($rustTarget -like "*apple*")   { return "libwhatsapp_bridge.dylib" }
    return "libwhatsapp_bridge.so"
}

function Get-FileSize([string]$path) {
    "{0:N0} KB" -f ((Get-Item $path).Length / 1KB)
}

# ── resolve target + profile ──────────────────────────────────────────────────

$rustTarget = if ($Target -ne "") { $Target } else { Get-HostTarget }
$profile    = if ($Debug) { "debug" } else { "release" }
$cargoProf  = if ($Debug) { "" }     else { "--release" }

$nativeDir   = Join-Path $PSScriptRoot "native"
$outDir      = Join-Path $nativeDir "target\$rustTarget\$profile"
$distNative  = Join-Path $PSScriptRoot "dist-native"

# Status values: "ok", "fail", "skip" — no special chars so comparisons work
$results  = @{ wasm = "skip"; c = "skip"; jni = "skip" }
$timings  = @{ wasm = ""; c = ""; jni = "" }
$startTotal = Get-Date

# ── 1. WASM + TypeScript ──────────────────────────────────────────────────────

if (-not $SkipWasm) {
    Write-Step "WASM + TypeScript"
    $t = Get-Date
    try {
        if (-not (Test-Path (Join-Path $PSScriptRoot "node_modules"))) {
            Invoke-Cmd "npm install" $PSScriptRoot
        }
        Invoke-Cmd "npm run build" $PSScriptRoot

        # wasm-pack -> pkg/    TypeScript -> dist/
        $wasmFile = Join-Path $PSScriptRoot "pkg\whatsapp_rust_bridge_bg.wasm"
        $jsFile   = Join-Path $PSScriptRoot "pkg\whatsapp_rust_bridge.js"
        if (Test-Path $wasmFile) { Write-Ok "pkg/whatsapp_rust_bridge_bg.wasm  ($(Get-FileSize $wasmFile))" }
        if (Test-Path $jsFile)   { Write-Ok "pkg/whatsapp_rust_bridge.js" }
        Write-Ok "dist/  (TypeScript)"

        $results["wasm"] = "ok"
        $timings["wasm"] = [math]::Round(((Get-Date) - $t).TotalSeconds, 1)
    } catch {
        Write-Err "WASM build failed: $_"
        $results["wasm"] = "fail"
    }
} else {
    Write-Info "WASM/TS skipped (-SkipWasm)"
}

# ── 2. C native ───────────────────────────────────────────────────────────────

if (-not $SkipC) {
    Write-Step "C Native Library  ($rustTarget  |  $profile)"

    $installed = rustup target list --installed 2>&1
    if ($installed -notcontains $rustTarget) {
        Write-Info "Installing Rust target $rustTarget ..."
        Invoke-Cmd "rustup target add $rustTarget"
    }

    $t = Get-Date
    try {
        Invoke-Cmd "cargo build $cargoProf --target $rustTarget" $nativeDir

        $libName = Get-LibName $rustTarget
        $libPath = Join-Path $outDir $libName

        # Copy into dist-native/c/
        $cDist = Join-Path $distNative "c"
        New-Item -ItemType Directory -Force -Path (Join-Path $cDist "include") | Out-Null
        Copy-Item $libPath                                        (Join-Path $cDist $libName)          -Force
        Copy-Item (Join-Path $nativeDir "include\whatsapp_bridge.h") (Join-Path $cDist "include\whatsapp_bridge.h") -Force

        Write-Ok "dist-native/c/$libName  ($(Get-FileSize (Join-Path $cDist $libName)))"
        Write-Ok "dist-native/c/include/whatsapp_bridge.h"

        $results["c"] = "ok"
        $timings["c"] = [math]::Round(((Get-Date) - $t).TotalSeconds, 1)
    } catch {
        Write-Err "C build failed: $_"
        $results["c"] = "fail"
    }
} else {
    Write-Info "C build skipped (-SkipC)"
}

# ── 3. Java / JNI ─────────────────────────────────────────────────────────────

if (-not $SkipJni) {
    Write-Step "Java / JNI Library  ($rustTarget  |  $profile)"

    $installed = rustup target list --installed 2>&1
    if ($installed -notcontains $rustTarget) {
        Write-Info "Installing Rust target $rustTarget ..."
        Invoke-Cmd "rustup target add $rustTarget"
    }

    $t = Get-Date
    try {
        Invoke-Cmd "cargo build $cargoProf --target $rustTarget --features jni" $nativeDir

        $libName  = Get-LibName $rustTarget
        $libPath  = Join-Path $outDir $libName

        # Copy into dist-native/jni/
        $jniDist     = Join-Path $distNative "jni"
        $javaDstRoot = Join-Path $jniDist "java\com\whatsapp\bridge"
        New-Item -ItemType Directory -Force -Path $jniDist    | Out-Null
        New-Item -ItemType Directory -Force -Path $javaDstRoot | Out-Null

        Copy-Item $libPath (Join-Path $jniDist $libName) -Force
        Write-Ok "dist-native/jni/$libName  ($(Get-FileSize (Join-Path $jniDist $libName)))"

        $javaSrc = Join-Path $nativeDir "java\com\whatsapp\bridge"
        Get-ChildItem $javaSrc -Filter "*.java" | ForEach-Object {
            Copy-Item $_.FullName (Join-Path $javaDstRoot $_.Name) -Force
            Write-Ok "dist-native/jni/java/com/whatsapp/bridge/$($_.Name)"
        }

        $results["jni"] = "ok"
        $timings["jni"] = [math]::Round(((Get-Date) - $t).TotalSeconds, 1)
    } catch {
        Write-Err "JNI build failed: $_"
        $results["jni"] = "fail"
    }
} else {
    Write-Info "JNI build skipped (-SkipJni)"
}

# ── Summary ───────────────────────────────────────────────────────────────────

$totalElapsed = [math]::Round(((Get-Date) - $startTotal).TotalSeconds, 1)

Write-Host ""
Write-Host "===========================================" -ForegroundColor DarkCyan
Write-Host "  Build Summary  (total: ${totalElapsed}s)" -ForegroundColor Cyan
Write-Host "===========================================" -ForegroundColor DarkCyan

foreach ($key in @("wasm", "c", "jni")) {
    $label = switch ($key) {
        "wasm" { "WASM + TypeScript" }
        "c"    { "C native         " }
        "jni"  { "Java / JNI       " }
    }
    $status = $results[$key]
    $t_str  = if ($timings[$key] -ne "") { "  ($($timings[$key])s)" } else { "" }

    switch ($status) {
        "ok"   { Write-Host ("  [OK]  {0}{1}" -f $label, $t_str) -ForegroundColor Green }
        "fail" { Write-Host ("  [ERR] {0}" -f $label) -ForegroundColor Red }
        "skip" { Write-Host ("  [ ]   {0}  skipped" -f $label) -ForegroundColor DarkGray }
    }
}

Write-Host ""
Write-Host "  Output locations:" -ForegroundColor DarkCyan

if ($results["wasm"] -eq "ok") {
    Write-Host "    WASM  :  pkg/whatsapp_rust_bridge_bg.wasm" -ForegroundColor Gray
    Write-Host "    TS    :  dist/" -ForegroundColor Gray
}
if ($results["c"] -eq "ok") {
    Write-Host "    C lib :  dist-native/c/$(Get-LibName $rustTarget)" -ForegroundColor Gray
    Write-Host "    Header:  dist-native/c/include/whatsapp_bridge.h" -ForegroundColor Gray
}
if ($results["jni"] -eq "ok") {
    Write-Host "    JNI   :  dist-native/jni/$(Get-LibName $rustTarget)" -ForegroundColor Gray
    Write-Host "    Java  :  dist-native/jni/java/com/whatsapp/bridge/*.java" -ForegroundColor Gray
}

Write-Host ""

if ($results.Values -contains "fail") { exit 1 }
