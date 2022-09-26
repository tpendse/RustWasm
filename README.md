# RustWasm
Experiments on WebAssembly in Rust

Rust WASM - Game of Life -> [Rust WASM beginner guide](https://rustwasm.github.io/docs/book/game-of-life/rules.html)

Running on Debian WSL

### What I need to do outside the tutorial...
1. Install `wasm-game-of-life` git repo manually
2. Could not get `rand` crate downloaded -- was downloading at 50 kbps 🤯
3. For tests - `wasm-pack test --chrome --headless` gave following errors
	1. Running first time `Error: failed to download from https://chromedriver.storage.googleapis.com/102.0.5005.61/chromedriver_linux64.zip`
	2. Downloaded `chromedriver` manually from given URL and placed unzipped `chromedriver` into `~/.cargo/bin/`
	3. Running command again gave `error while loading shared libraries: libnss3.so: cannot open shared object file: No such file or directory`
	4. Updated `apt` using `sudo apt update` then installed `libnss3` using `sudo apt install libnss3-dev`
	5. Build & Run succeeded -- but gave test errors


---
__More to come!__
