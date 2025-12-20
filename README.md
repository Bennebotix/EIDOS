# EIDOS: Ellipse-based Image Desmos Optimization Solver

EIDOS takes any image and reconstructs it using hundreds or thousands of overlapping ellipses. It then spits out a single JSON blob (JS) you can paste into the Desmos Dev Console to load the output.

The heavy lifting is done in Rust (compiled to WASM) so it's fast enough to run in your browser.

[Demo](https://bennebotix.github.io/EIDOS/dist/index.html)
[Example](https://desmos.com/calculator/)

## 🚀 Features

- **Fast & Lightweight**: Rust + WASM handles the math; JS handles the UI.
- **Hill-Climbing Algorithm**: It iteratively optimizes every shape to match the original image as closely as possible.
- **Fidelity Modes**: From "quick sketch" to "extreme detail" (standard to hyper). THis changes how long the algorithm runs when finding the best ellipse to add.
- **Single-File Bundle**: You can bundle the entire app into a single `index.html` and run it offline.

## 🛠 Prerequisites

1. **Rust & cargo**: Install via [rustup.rs](https://rustup.rs/).
2. **wasm-pack**: This is what builds the Rust code for the web.
   ```bash
   cargo install wasm-pack
   ```
3. **Node.js**: Needed for the Vite dev server. Download from [nodejs.org](https://nodejs.org/).

## 📦 Setup & Usage

### 1. Build the Rust Core

**Windows:**
```powershell
.\build.ps1
```

**Linux/macOS:**
```bash
./build.sh
```

### 2. Run the Web App
This starts a local dev server.

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

## 🧬 How it works (The Math)

EIDOS uses a **Hill-Climbing** algorithm. Instead of trying to "understand" the image, it starts with a blank canvas and tries to find the best place for a new ellipse.

1. **Seed**: Find the pixel with the most "error" (where the current state is most different from the target).
2. **Mutate**: Throw a bunch of random ellipses at that spot. Move them, rotate them, and resize them.
3. **Score**: Calculate which change reduces the total error the most.
4. **Repeat**: Do this thousands of times.

The resulting shapes are converted into LaTeX inequalities that look like this:
`\frac{(x \cos a + y \sin a)^2}{rx^2} + \frac{(-x \sin a + y \cos a)^2}{ry^2} \le 1`

---
Created by [Bennett Lang (Bennebotix)](https://github.com/Bennebotix)
