#!/bin/bash
set -e

echo "Bundling EIDOS into a single HTML file..."

HTML_PATH="web/index.html"
CSS_PATH="web/src/style.css"
JS_PATH="web/src/main.js"
WASM_JS_PATH="rust/pkg/rust_core.js"
WASM_PATH="rust/pkg/rust_core_bg.wasm"
DIST_DIR="dist"
DIST_PATH="dist/index.html"

mkdir -p "$DIST_DIR"

# Read files
HTML_CONTENT=$(cat "$HTML_PATH")
CSS_CONTENT=$(cat "$CSS_PATH")
WASM_JS_CONTENT=$(cat "$WASM_JS_PATH")
MAIN_JS_CONTENT=$(cat "$JS_PATH")

# Encode WASM to Base64
# Note: base64 command varies slightly between Linux (coreutils) and macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    BASE64_WASM=$(base64 -i "$WASM_PATH")
else
    BASE64_WASM=$(base64 -w 0 "$WASM_PATH")
fi

# Clean up WASM JS
# Remove exports and imports
WASM_JS_CONTENT=$(echo "$WASM_JS_CONTENT" | sed 's/export default .*;//g' | sed 's/export {.*};//g' | sed 's/import .* from .*;//g')
WASM_JS_CONTENT+=$'\n'"const init = __wbg_init;"

# Clean up Main JS
MAIN_JS_CONTENT=$(echo "$MAIN_JS_CONTENT" | sed 's/import .* from .*;//g')

# Build the bundle script tag
BUNDLE_SCRIPT="<script type=\"module\">
// --- WASM Bindings ---
$WASM_JS_CONTENT

// --- WASM Base64 ---
const wasmBase64 = \"$BASE64_WASM\";

function base64ToUint8Array(base64) {
    var binary_string = window.atob(base64);
    var len = binary_string.length;
    var bytes = new Uint8Array(len);
    for (var i = 0; i < len; i++) {
        bytes[i] = binary_string.charCodeAt(i);
    }
    return bytes;
}

console.log(\"Initializing embedded WASM...\");
await init(base64ToUint8Array(wasmBase64));
console.log(\"WASM Initialized.\");

// --- Main Application Logic ---
$MAIN_JS_CONTENT
</script>"

# Replace and write to dist
# Using a temp file for simple replacement
TEMP_HTML=$(mktemp)
echo "$HTML_CONTENT" > "$TEMP_HTML"

# Inline CSS (Replace the link tag with the style tag)
# We use a bit of python here to avoid shell quoting/sed hell for large multi-line strings
python3 -c "
import sys
html = open('$TEMP_HTML').read()
css = open('$CSS_PATH').read()
js = '''$BUNDLE_SCRIPT'''
html = html.replace('<link rel=\"stylesheet\" href=\"./src/style.css\" />', f'<style>\\n{css}\\n</style>')
html = html.replace('<script type=\"module\" src=\"./src/main.js\"></script>', js)
with open('$DIST_PATH', 'w') as f:
    f.write(html)
"

echo "Bundle created at $DIST_PATH"
# Using python for file size to be cross-platform easily
python3 -c "import os; size = os.path.getsize('$DIST_PATH')/1024/1024; print(f'Size: {size:.2f} MB')"
rm "$TEMP_HTML"
