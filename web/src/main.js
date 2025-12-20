import init, { DesmosOptimizer } from '../../rust/pkg/rust_core.js';

const LOG_INTERVAL = 50;
const FRAME_TIME_MS = 12;
const STANDARD_BATCH_SIZE = 25;
const HIGH_FIDELITY_BATCH_SIZE = 10;
const HYPER_FIDELITY_BATCH_SIZE = 2;
const UI_YIELD_MS = 50;
const COPY_FEEDBACK_DURATION_MS = 2000;

const dropZone = document.getElementById('dropZone');
const fileInput = document.getElementById('fileInput');
const preview = document.getElementById('preview');
const processBtn = document.getElementById('processBtn');
const output = document.getElementById('output');
const shapeCountSlider = document.getElementById('shapeCount');
const shapeCountVal = document.getElementById('shapeCountVal');
const logConsole = document.getElementById('logConsole');

let currentFileData = null;

async function run() {
    await init();
    log("WASM Core Loaded. Ready.");
}

run();

function log(msg, type = 'system') {
    const div = document.createElement('div');
    div.className = `log-line ${type}`;
    div.innerText = `> ${msg}`;
    logConsole.appendChild(div);
    logConsole.scrollTop = logConsole.scrollHeight;
}

const originalLog = console.log;
console.log = function(...args) {
    originalLog.apply(console, args);
    if (typeof args[0] === 'string') {
        log(args[0], 'info');
    }
};

if (shapeCountSlider) {
    shapeCountSlider.addEventListener('input', (e) => {
        shapeCountVal.innerText = e.target.value;
    });
}

if (dropZone) {
    dropZone.addEventListener('dragover', (e) => {
        e.preventDefault();
        dropZone.style.background = 'rgba(255,255,255,0.05)';
    });
    dropZone.addEventListener('dragleave', (e) => {
        e.preventDefault();
        dropZone.style.background = 'transparent';
    });
    dropZone.addEventListener('drop', (e) => {
        e.preventDefault();
        dropZone.style.background = 'transparent';
        handleFile(e.dataTransfer.files[0]);
    });
    dropZone.addEventListener('click', () => fileInput.click());
}

if (fileInput) {
    fileInput.addEventListener('change', (e) => handleFile(e.target.files[0]));
}

function handleFile(file) {
    if (!file || !file.type.startsWith('image/')) return;
    
    const reader = new FileReader();
    reader.onload = (e) => {
        if (preview) {
            preview.src = e.target.result;
            preview.classList.remove('hidden');
        }
        document.querySelector('.drop-content')?.classList.add('hidden');
    };
    
    const bufReader = new FileReader();
    bufReader.onload = (e) => {
        currentFileData = new Uint8Array(e.target.result);
        if (processBtn) processBtn.disabled = false;
        log(`Image loaded: ${file.name} (${(file.size/1024).toFixed(1)} KB)`);
    };
    bufReader.readAsArrayBuffer(file);
    
    reader.readAsDataURL(file);
}

const clearLogsBtn = document.getElementById('clearLogs');
if (clearLogsBtn) {
    clearLogsBtn.addEventListener('click', () => {
        logConsole.innerHTML = '';
        log("Logs cleared.", 'system');
    });
}

const copyBtn = document.getElementById('copyBtn');
if (copyBtn) {
    copyBtn.addEventListener('click', () => {
        output.select();
        navigator.clipboard.writeText(output.value);
        const originalText = copyBtn.innerText;
        copyBtn.innerText = "Copied!";
        setTimeout(() => copyBtn.innerText = originalText, COPY_FEEDBACK_DURATION_MS);
    });
}


function setButtonText(text) {
    processBtn.innerHTML = `<span class="btn-text">${text}</span><div class="btn-glow"></div>`;
}

if (processBtn) {
    processBtn.addEventListener('click', async () => {
        if (!currentFileData) return;
        
        const shapes = parseInt(shapeCountSlider.value);
        const detailLevel = document.getElementById('detailLevel').value;
        let fidelityMode = 0;
        if (detailLevel === 'high') fidelityMode = 1;
        if (detailLevel === 'super') fidelityMode = 2;
        if (detailLevel === 'hyper') fidelityMode = 3;
        
        log(`Starting optimization... (${shapes} shapes, Mode: ${detailLevel})`);
        processBtn.disabled = true;
        processBtn.innerHTML = '<span class="btn-text">Initializing...</span>';
        output.value = "";
        
        await new Promise(r => setTimeout(r, UI_YIELD_MS));

        try {
            let optimizer = new DesmosOptimizer(currentFileData, shapes, fidelityMode);
            let done = false;
            let batchSize = STANDARD_BATCH_SIZE;
            if (fidelityMode === 1 || fidelityMode === 2) batchSize = HIGH_FIDELITY_BATCH_SIZE;
            if (fidelityMode === 3) batchSize = HYPER_FIDELITY_BATCH_SIZE;
            
            processBtn.innerHTML = `<span class="btn-text">Optimizing...</span>`;
            
            async function evolve() {
                try {
                    const startTime = performance.now();
                    
                    while (performance.now() - startTime < FRAME_TIME_MS) {
                        done = optimizer.step(batchSize);
                        if (done) break;
                    }

                    if (!done) {
                        requestAnimationFrame(evolve);
                    } else {
                        log(`Added shape ${shapes}/${shapes}`);
                        finish();
                    }
                } catch (e) {
                    log(`Optimization Error: ${e}`, 'warning');
                    resetBtn();
                }
            }
            
            function finish() {
                log("Optimization Complete. Generating final JSON...");
                const json = optimizer.get_json();
                
                const sizeBytes = new TextEncoder().encode(json).length;
                const sizeMB = (sizeBytes / (1024 * 1024)).toFixed(2);
                log(`Generated Cloud Payload Size: ${sizeMB} MB`);
                
                const wrapped = `Calc.setState(${json});`;
                output.value = wrapped;
                log("Ready. Use the Copy button to copy to clipboard.");
                
                optimizer.free();
                resetBtn();
            }

            function resetBtn() {
                processBtn.disabled = false;
                setButtonText('Compile Image');
            }
            
            evolve();
            
        } catch (err) {
            log(`Error: ${err}`, 'warning');
            console.error(err);
            processBtn.disabled = false;
            processBtn.innerHTML = '<span class="btn-text">Compile Image</span>';
        }
    });
}
