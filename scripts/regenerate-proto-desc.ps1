# Regenerate internal/waproto/src/whatsapp.desc from whatsapp.proto.
#
# Under buffa, `cargo build` reads the committed .desc and writes the Rust
# source to OUT_DIR — consumers never need protoc. Only editing the .proto
# requires this script; commit whatsapp.proto, whatsapp.desc and
# whatsapp.desc.sha256 together (build.rs verifies the hashes match).
#
# Uses the protoc bundled by the protoc-bin-vendored crate, so nothing has to
# be installed on PATH.

$ErrorActionPreference = "Stop"

$root  = Split-Path -Parent $PSScriptRoot
$src   = Join-Path $root "internal\waproto\src"
$proto = Join-Path $src "whatsapp.proto"
$desc  = Join-Path $src "whatsapp.desc"
$hash  = Join-Path $src "whatsapp.desc.sha256"

$protoc = Get-ChildItem "$env:USERPROFILE\.cargo\registry\src" -Recurse -Filter protoc.exe -ErrorAction SilentlyContinue |
          Select-Object -First 1 -ExpandProperty FullName
if (-not $protoc) {
    $protoc = (Get-Command protoc -ErrorAction SilentlyContinue).Source
}
if (-not $protoc) { throw "protoc not found (build once so protoc-bin-vendored is fetched, or install protoc)" }

Write-Host "protoc: $protoc"
& $protoc --descriptor_set_out="$desc" --include_imports --include_source_info -I"$src" "$proto"
if ($LASTEXITCODE -ne 0) { throw "protoc failed ($LASTEXITCODE)" }

function Get-Sha256([string]$path) {
    (Get-FileHash -Algorithm SHA256 -Path $path).Hash.ToLower()
}

# build.rs expects exactly: "proto <sha>\ndesc <sha>\n" (LF endings)
$content = "proto $(Get-Sha256 $proto)`ndesc $(Get-Sha256 $desc)`n"
[IO.File]::WriteAllText($hash, $content)

Write-Host "regenerated:"
Write-Host "  $desc"
Write-Host "  $hash"
