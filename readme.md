# Description

this mod add [wasm](https://webassembly.org/) support for
[CC: Tweaked](https://www.curseforge.com/minecraft/mc-mods/cc-tweaked).

this mod uses [chicory runtime](https://github.com/dylibso/chicory) (v1.7.5),
and includes [wasi](https://wasi.dev/) support.

# Mounted directories

- `wasm` — global program dir, read-only, `.minecraft/wasm/`
- `wasm_save` — per-save program dir, read-only, `.minecraft/saves/<world>/computercraft/wasm/`
- `shared` — per-save writable data dir (1 GB quota), `.minecraft/saves/<world>/computercraft/shared/`

# Usage

put `.wasm` files (compiled from Rust, see [cc_wasm_api](https://crates.io/crates/cc_wasm_api))
into the `wasm` folder (`.minecraft/wasm/`), then in game:

## commands (shell)

- `exec_wasm <name>` — load wasm (interpreter) and drive its coroutine loop (`tick`/`stopped`)
- `exec_wasm_aot <name>` — load wasm with AOT + disk cache, then drive the coroutine loop
- `wasm_compile_aot <name> [global|save|auto]` — precompile a wasm to the AOT disk cache in background

## lua api

- `wasm.load_wasm(name, [useAoT], [source])` — load a module (no `.wasm` extension).
  `useAoT` defaults to `true`; `source` is `"auto"` (save first) / `"global"` / `"save"`.
- `wasm.precompile(name, [source])` — start background AOT compilation (thread `ccwasm-aot`),
  writes result to the disk cache. Needed for large modules, because a single Lua call is
  limited by the computer thread timeout (~7.5s), while AOT compiling a big module
  (e.g. the python example) takes tens of seconds.
- `wasm.precompile_done(name)` — poll status: `"compiling"` / `"done"` / `"failed"` / `nil`.
  In Lua, poll with `os.sleep` to avoid the timeout.

after a module is precompiled once, `wasm.load_wasm(name, true)` loads it from the cache
in ~1.4s no matter how big it is.

# AOT disk cache

compiled machine code is cached in `.minecraft/wasm_cache/` (keyed by the wasm content sha256,
so changing the wasm invalidates the cache automatically; corrupted cache falls back to a
cache-less recompile).

# Wasm development

use this [crate](https://crates.io/crates/cc_wasm_api).

features (opt-in, Cargo features):
- `coroutine` — `tick`/`stopped` exports for driving a loop with `exec_wasm`/`exec_wasm_aot`
- `eval` — call back into Lua (`eval_string`/`eval_result`)
- `addon` — local monitor / vec2d / time helpers
- `debug` — debug output helpers

see the examples [here](https://github.com/wefcdse/ccwasm/tree/master/examples):
minesweeper, pic_display, txt_display, python, obj_demo, type_test.

# Example

[minesweeper example](https://github.com/wefcdse/ccwasm/blob/master/example_wasm/minesweeper.wasm)
- download this file and put it in `.minecraft/wasm/`

- place a monitor on top of a computer

- in computer craft's shell run `exec_wasm minesweeper`

[picture display example](https://github.com/wefcdse/ccwasm/blob/master/example_wasm/pic_display.wasm)
- download this file and put it in `.minecraft/wasm/`

- place a monitor on top of a computer

- in computer craft's shell run `exec_wasm pic_display [path to picture file(in cc's fs)]`

[python interpreter example](https://github.com/wefcdse/ccwasm/blob/master/example_wasm/python.wasm),
download this file and put it in `./wasm/`.

python.wasm is big (9.6MB): AOT compiling it takes ~11s, which exceeds the computer thread
timeout for a single Lua call, so **precompile it first** (runs in a background thread and
writes the AOT disk cache):
```
wasm_compile_aot python
```
then in computer craft lua run:
```lua
py = wasm.load_wasm("python") -- AOT, loads from disk cache in ~1.4s
py.init()
py.exec("import time as t")
print(py.eval("t.time()"))
```

# Tests

test scripts live in `wasm/test/` (dev run dir), entry point `test.lua` (run `dofile` from a
computer, e.g. `dofile("wasm/test/test.lua")`). Results are written to `shared/`.
