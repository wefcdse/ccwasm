cargo build -r --lib --target wasm32-wasip1
del ..\..\run\wasm\py.wasm
copy .\target\wasm32-wasip1\release\py.wasm ..\..\run\wasm\py.wasm
