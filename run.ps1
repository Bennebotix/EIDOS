$ErrorActionPreference = "Stop"

Write-Host "Starting Web Server..."
Set-Location "web"
npm run dev
