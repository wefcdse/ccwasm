# Description
this mod add [wasm](https://webassembly.org/) support for
[CC: Tweaked](https://www.curseforge.com/minecraft/mc-mods/cc-tweaked).

this mod uses [chicory runtime](https://github.com/dylibso/chicory),
and includes [wasi](https://wasi.dev/) support



# Usage
- put `.wasm` file in `wasm` folder.
- in lua in computer craft, run `wasm.load_wasm` function
to load a module. Note that the argument does not contains a `.wasm` extend name.

# Wasm development
use this [crate](https://crates.io/crates/cc_wasm_api)

see the example [here](https://github.com/wefcdse/ccwasm/tree/master/wasmlib)

# Example
[minesweeper example](https://github.com/wefcdse/ccwasm/tree/master/examples)

[wasm file](https://github.com/wefcdse/ccwasm/blob/master/example_wasm/minesweeper.wasm),

- download this file and put it in `./wasm/`

- place a monitor on top of a computer

- in computer craft's shell run `exec_wasm_auto_reload minesweeper`

there is [a python interpreter example](https://github.com/wefcdse/ccwasm/blob/master/example_wasm/python.wasm), 
download this file and put it in `./wasm/`, and then in computer craft lua run:
```lua
py = wasm.load_wasm("python")
py.init()
py.exec("import time as t")
print(py.eval("t.time()"))
```
