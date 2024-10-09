cargo build -r --lib --target wasm32-wasi
wasm2wat.exe .\target\wasm32-wasi\release\minesweeper.wasm -o out.wat
del ..\..\run\wasm\minesweeper.wasm
@REM copy .\target\wasm32-unknown-unknown\release\wasmlib.wasm ..\run\wasm\a.wasm
copy .\target\wasm32-wasi\release\minesweeper.wasm ..\..\run\wasm\minesweeper.wasm
