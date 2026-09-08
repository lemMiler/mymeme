$ErrorActionPreference = "Stop"
$target = "x86_64-pc-windows-msvc"

python tools/validate_resources.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

rustup target add $target
cargo check --target $target
cargo build --release --target $target

$stage = Join-Path $PSScriptRoot "dist/MEME_HOME"
Remove-Item (Join-Path $PSScriptRoot "dist") -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force (Join-Path $stage "libraries") | Out-Null
New-Item -ItemType Directory -Force (Join-Path $stage "resources") | Out-Null

Copy-Item (Join-Path $PSScriptRoot "target/$target/release/memelite_custom_memes.dll") (Join-Path $stage "libraries/")
Copy-Item (Join-Path $PSScriptRoot "resources/images") (Join-Path $stage "resources/images") -Recurse
Copy-Item (Join-Path $PSScriptRoot "config.example.toml") (Join-Path $stage "config.example.toml")

$zip = Join-Path $PSScriptRoot "dist/memelite_custom_memes-windows-x64.zip"
Compress-Archive -Path (Join-Path $stage "*") -DestinationPath $zip -Force
Write-Host "Build complete: $zip"
