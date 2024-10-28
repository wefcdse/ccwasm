@REM cargo run 
cargo build -r --lib --target wasm32-wasip1
copy .\target\wasm32-wasip1\release\*.wasm ..\..\run\wasm\
