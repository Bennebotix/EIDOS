$ErrorActionPreference = "Stop"

Write-Host "Building Rust Core..."
Set-Location "rust"
wasm-pack build --target web
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Set-Location ..

Write-Host "Build Complete."
