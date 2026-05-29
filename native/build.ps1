# Build the native bridge for the current host platform.
# Usage:
#   .\build.ps1              # debug build
#   .\build.ps1 --release    # release build
#   .\build.ps1 --jni        # include JNI bindings (for Java/Kotlin/Android)

param(
    [switch]$Release,
    [switch]$Jni
)

$features = if ($Jni) { "--features jni" } else { "" }
$profile  = if ($Release) { "--release" } else { "" }

# Detect host target
$arch = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture.ToString().ToLower()
$os   = [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Windows) ? "windows" : (
        [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::OSX) ? "macos" : "linux")

$target = switch ("$os-$arch") {
    "windows-x64"   { "x86_64-pc-windows-msvc" }
    "windows-arm64" { "aarch64-pc-windows-msvc" }
    "macos-x64"     { "x86_64-apple-darwin" }
    "macos-arm64"   { "aarch64-apple-darwin" }
    "linux-x64"     { "x86_64-unknown-linux-gnu" }
    "linux-arm64"   { "aarch64-unknown-linux-gnu" }
    default         { throw "Unknown platform: $os-$arch" }
}

Write-Host "Building for target: $target" -ForegroundColor Cyan
$cmd = "cargo build $profile --target $target $features"
Write-Host $cmd -ForegroundColor Gray
Invoke-Expression $cmd

if ($LASTEXITCODE -eq 0) {
    $profileDir = if ($Release) { "release" } else { "debug" }
    $outDir = "target\$target\$profileDir"
    Write-Host ""
    Write-Host "Output: $outDir" -ForegroundColor Green
    Get-ChildItem $outDir | Where-Object { $_.Extension -in ".dll",".so",".dylib",".lib",".a" } | ForEach-Object {
        Write-Host "  $($_.Name)" -ForegroundColor Green
    }
    Write-Host ""
    Write-Host "C header: include\whatsapp_bridge.h" -ForegroundColor Green
    if ($Jni) {
        Write-Host "Java sources: java\com\whatsapp\bridge\" -ForegroundColor Green
    }
}
