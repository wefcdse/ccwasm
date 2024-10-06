@REM cargo build --target wasm32-unknown-unknown
@REM cargo build -r --lib --target wasm32-unknown-unknown
cargo build -r --lib --target wasm32-wasi
@REM cargo build --target wasm32-wasi --no-default-features --features freeze-stdlib,stdlib --release

@REM wasm2wat.exe .\target\wasm32-unknown-unknown\debug\wasmlib.wasm -o out.wat
@REM wasm2wat.exe .\target\wasm32-wasi\debug\wasmlib.wasm -o out.wat
@REM wasm2wat.exe .\target\wasm32-wasi\release\wasmlib.wasm -o out.wat
@REM wasm2wat.exe .\target\wasm32-unknown-unknown\release\wasmlib.wasm -o out.wat
wasm2wat.exe .\target\wasm32-wasi\release\wasmlib.wasm -o out.wat
@REM wat2wasm.exe .\out.wat -o ..\a.wasm

@REM copy .\target\wasm32-wasi\debug\wasmlib.wasm ..\a.wasm

del ..\a.wasm
@REM copy .\target\wasm32-unknown-unknown\release\wasmlib.wasm ..\a.wasm
copy .\target\wasm32-wasi\release\wasmlib.wasm ..\a.wasm

del ..\run\wasm\a.wasm
@REM copy .\target\wasm32-unknown-unknown\release\wasmlib.wasm ..\run\wasm\a.wasm
copy .\target\wasm32-wasi\release\wasmlib.wasm ..\run\wasm\a.wasm
