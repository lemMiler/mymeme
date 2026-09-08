$ErrorActionPreference = "Stop"
$target = "x86_64-pc-windows-msvc"
$requiredRust = "1.93.1"

Write-Host "Checking resources..."
python tools/validate_resources.py
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Installing/pinning Rust $requiredRust..."
rustup toolchain install $requiredRust --profile minimal
rustup target add $target --toolchain $requiredRust
$rustVersion = (& rustc "+$requiredRust" --version)
Write-Host "rustc: $rustVersion"
if ($rustVersion -notmatch '^rustc 1\.93\.1 ') {
    throw "Wrong Rust toolchain: $rustVersion"
}

cargo "+$requiredRust" check --target $target
cargo "+$requiredRust" build --release --target $target

$stage = Join-Path $PSScriptRoot "dist/MEME_HOME"
Remove-Item (Join-Path $PSScriptRoot "dist") -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force (Join-Path $stage "libraries") | Out-Null
New-Item -ItemType Directory -Force (Join-Path $stage "resources/images") | Out-Null

Copy-Item (Join-Path $PSScriptRoot "target/$target/release/memelite_custom_memes.dll") (Join-Path $stage "libraries/")
Copy-Item (Join-Path $PSScriptRoot "resources/images/*") (Join-Path $stage "resources/images/") -Recurse -Force
Copy-Item (Join-Path $PSScriptRoot "config.example.toml") (Join-Path $stage "config.example.toml")
@(
    "meme-generator target: 0.2.3",
    "rustc target: 1.93.1",
    "target: $target",
    "NOTE: local build_windows.ps1 does not run the Python host ABI smoke test; GitHub Actions does."
) | Set-Content (Join-Path $stage "BUILD_INFO.txt") -Encoding utf8

$zip = Join-Path $PSScriptRoot "dist/memelite_custom_memes-windows-x64.zip"
Compress-Archive -Path (Join-Path $stage "*") -DestinationPath $zip -Force
Write-Host "Build complete: $zip"
Write-Host "For the safest package, use the GitHub Actions artifact because CI also runs a real meme-generator 0.2.3 ABI smoke test."
