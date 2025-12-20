
Write-Host "Bundling EIDOS into a single HTML file..."

$htmlPath = "web/index.html"
$cssPath = "web/src/style.css"
$jsPath = "web/src/main.js"
$wasmJsPath = "rust/pkg/rust_core.js"
$wasmPath = "rust/pkg/rust_core_bg.wasm"
$distDir = "dist"
$distPath = "dist/index.html"

if (!(Test-Path $distDir)) {
    New-Item -ItemType Directory -Force -Path $distDir | Out-Null
}

$html = Get-Content $htmlPath -Raw
$css = Get-Content $cssPath -Raw
$wasmJs = Get-Content $wasmJsPath -Raw
$mainJs = Get-Content $jsPath -Raw

# Inlining CSS
$html = $html.Replace('<link rel="stylesheet" href="./src/style.css" />', "<style>`n$css`n</style>")

# Prepare WASM
if (!(Test-Path $wasmPath)) {
    Write-Error "WASM file not found at $wasmPath. Please run build.ps1 first."
    exit 1
}
$wasmBytes = [System.IO.File]::ReadAllBytes($wasmPath)
$base64Wasm = [Convert]::ToBase64String($wasmBytes)

# Process WASM JS Bindings
# Remove all exports and imports to make it a plain script
$wasmJs = $wasmJs -replace 'export\s+default\s+[^;]+;', ''
$wasmJs = $wasmJs -replace 'export\s+{[^}]+};', ''
$wasmJs = $wasmJs -replace 'import\s+.*\s+from\s+[^;]+;', ''

# Explicitly alias the init function
$wasmJs += "`nconst init = __wbg_init;`n"

# Process Main JS
$mainJs = $mainJs -replace 'import\s+.*\s+from\s+[^;]+;', ''

$bundleScript = @"
<script type="module">
// --- WASM Bindings ---
$wasmJs

// --- WASM Base64 ---
const wasmBase64 = "$base64Wasm";

function base64ToUint8Array(base64) {
    var binary_string = window.atob(base64);
    var len = binary_string.length;
    var bytes = new Uint8Array(len);
    for (var i = 0; i < len; i++) {
        bytes[i] = binary_string.charCodeAt(i);
    }
    return bytes;
}

console.log("Initializing embedded WASM...");
await init(base64ToUint8Array(wasmBase64));
console.log("WASM Initialized.");

// --- Main Application Logic ---
$mainJs

</script>
"@

$html = $html.Replace('<script type="module" src="./src/main.js"></script>', $bundleScript)

Set-Content -Path $distPath -Value $html -Encoding utf8

Write-Host "Bundle created at $distPath"
$size = (Get-Item $distPath).Length / 1MB
Write-Host ("Size: {0:N2} MB" -f $size)
