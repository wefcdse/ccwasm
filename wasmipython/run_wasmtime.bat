cargo build -r --target wasm32-wasi
wasm2wat.exe .\target\wasm32-wasi\release\wasmipython.wasm -o out.wat


wasmtime.exe .\target\wasm32-wasi\release\wasmipython.wasm

