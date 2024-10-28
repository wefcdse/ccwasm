cargo build -r --lib --target wasm32-wasi
@REM wasm2wat.exe .\target\wasm32-wasi\release\txt_display.wasm -o out.wat
del ..\..\run\wasm\txt_display.wasm
@REM copy .\target\wasm32-unknown-unknown\release\wasmlib.wasm ..\run\wasm\a.wasm
copy .\target\wasm32-wasi\release\txt_display.wasm ..\..\run\wasm\txt_display.wasm
