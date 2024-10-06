cargo build -r --lib --target wasm32-wasi

wasm2wat.exe .\target\wasm32-wasi\release\py.wasm -o out.wat


del ..\py.wasm
copy .\target\wasm32-wasi\release\py.wasm ..\py.wasm

