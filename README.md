# EIDOS: Ellipse-based Image Desmos Optimization Solver

EIDOS takes any image and reconstructs it using hundreds or thousands of overlapping ellipses. It then spits out a single JSON blob (JS) you can paste into the Dev Console on Desmos to paste in the image.

The heavy lifting is done in Rust (compiled to WASM) so it's fast enough to run in your browser.

[Demo](https://bennebotix.github.io/EIDOS/dist/index.html)
[Example](https://desmos.com/calculator/)

## Prerequisites

1. **Rust & cargo**: Install via [rustup.rs](https://rustup.rs/).
2. **wasm-pack**: This is what builds the Rust code for the web.
   ```bash
   cargo install wasm-pack
   ```
3. **Node.js**: Needed for the Vite dev server. Download from [nodejs.org](https://nodejs.org/).

## Setup & Usage

### 1. Build the Rust part

**Windows:**
```powershell
.\build.ps1
```

**Linux/macOS:**
```bash
./build.sh
```

### 2. Run the Web App
This starts a local server.

**Windows:**
```powershell
.\run.ps1
```

**Linux/macOS:**
```bash
./run.sh
```

Navigate to `http://localhost:5173` (or whatever Vite tells you).

### 3. Creating a Portable Bundle
If you want to create a static single-file version of the app:

**Windows:**
```powershell
.\bundle.ps1
```

**Linux/macOS:**
```bash
./bundle.sh
```
The result will be in `dist/index.html`.

### Fidelity Modes

| **Mode** | **Multiplier*** | **Total Random Trials** | **Total Hill Climb Steps** |
|----------|-----------------|-------------------------|----------------------------|
| Standard | 3x              | 120                     | 240                        |
| High     | 10x             | 400                     | 800                        |
| Hyper    | 100x            | 4,000                   | 8,000                      |

\* Multiplier means how many times the computations are repeated from a satndard batch (increased to 3x for looks).


The resulting shapes are converted into LaTeX inequalities that look like this:
`\frac{(x \cos a + y \sin a)^2}{rx^2} + \frac{(-x \sin a + y \cos a)^2}{ry^2} \le 1`

---

Created by [Bennett Lang (Bennebotix)](https://github.com/Bennebotix)
