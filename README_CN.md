# 描述

这个 mod 为 [CC: Tweaked](https://www.curseforge.com/minecraft/mc-mods/cc-tweaked) 添加了
[WebAssembly](https://webassembly.org/) 支持。

运行时使用 [chicory](https://github.com/dylibso/chicory)（v1.7.5），
支持 [wasi](https://wasi.dev/)。

# 挂载目录

- `wasm` — 全局程序目录，只读，对应 `.minecraft/wasm/`
- `wasm_save` — 存档专属程序目录，只读，对应 `.minecraft/saves/<世界>/computercraft/wasm/`
- `shared` — 存档专属可写数据目录（配额 1GB），对应 `.minecraft/saves/<世界>/computercraft/shared/`

# 用法

把编译好的 `.wasm` 文件（用 Rust 编写，见 [cc_wasm_api](https://crates.io/crates/cc_wasm_api)）
放进 `wasm` 目录（`.minecraft/wasm/`），然后在游戏里：

## 命令（shell）

- `exec_wasm <名字>` — 加载 wasm（解释模式）并驱动它的协程循环（`tick`/`stopped`）
- `exec_wasm_aot <名字>` — 用 AOT + 磁盘缓存加载 wasm，然后驱动协程循环
- `wasm_compile_aot <名字> [global|save|auto]` — 后台把 wasm 预编译到 AOT 磁盘缓存

## lua api

- `wasm.load_wasm(name, [useAoT], [source], [useStdio])` — 加载模块（不带 `.wasm` 扩展名）。
  `useAoT` 默认 `true`；`source` 为 `"auto"`（存档优先）/ `"global"` / `"save"`；
  `useStdio = true` 时模块多三个方法：
  - `stdin(...)` — 把字符串推入 WASI stdin（stdin 空 = EOF，不会阻塞）
  - `stdout()` — 取走自上次调用以来写入 stdout 的所有输出（并清空）
  - `stderr()` — 同上，针对 stderr
- `wasm.precompile(name, [source])` — 启动后台 AOT 编译（线程 `ccwasm-aot`），
  结果写入磁盘缓存。大模块必需：单次 Lua 调用受电脑线程超时限制（约 7.5 秒），
  而 AOT 编译大模块（如 python 示例）需要几十秒。
- `wasm.precompile_done(name)` — 轮询状态：`"compiling"` / `"done"` / `"failed"` / `nil`。
  Lua 侧用 `os.sleep` 轮询即可避开超时。

模块预编译一次后，`wasm.load_wasm(name, true)` 无论多大都约 1.4 秒从缓存加载。

# AOT 磁盘缓存

编译产物缓存在 `.minecraft/wasm_cache/`（key 为 wasm 内容的 sha256，
wasm 改动自动失效；缓存损坏时回退到无缓存重新编译）。

# Wasm 开发

使用这个 [crate](https://crates.io/crates/cc_wasm_api)。

features（全部可选，默认 = coroutine + eval + addon）：
- `coroutine` — 提供 `tick`/`stopped` 导出，供 `exec_wasm`/`exec_wasm_aot` 驱动事件循环
- `eval` — 回调进 Lua（`eval_string`/`eval_result` 在 wasm 与 Lua 之间往返）
- `addon` — 本地显示器（`LocalMonitor`）、`vec2d`、`time`、`ColorId` 等工具
- `debug` — 调试输出工具（`show_str`/`show_debug`）

示例见 [这里](https://github.com/wefcdse/ccwasm/tree/master/examples)：
minesweeper、pic_display、txt_display、python、obj_demo、type_test。

# 示例

[minesweeper 示例](https://github.com/wefcdse/ccwasm/blob/master/example_wasm/minesweeper.wasm)
- 下载此文件并放入 `.minecraft/wasm/`

- 在电脑上方放置一个显示器

- 在电脑 shell 中运行 `exec_wasm minesweeper`

[picture display 示例](https://github.com/wefcdse/ccwasm/blob/master/example_wasm/pic_display.wasm)
- 下载此文件并放入 `.minecraft/wasm/`

- 在电脑上方放置一个显示器

- 在电脑 shell 中运行 `exec_wasm pic_display [图片路径(cc 的 fs 内)]`

[python 解释器示例](https://github.com/wefcdse/ccwasm/blob/master/example_wasm/python.wasm)，
下载此文件并放入 `./wasm/`。

python.wasm 很大（9.6MB）：AOT 编译约需 11 秒，超过电脑线程单次 Lua 调用的超时，
所以**先预编译**（在后台线程运行并写入 AOT 磁盘缓存）：
```
wasm_compile_aot python
```
然后在电脑 lua 中运行：
```lua
py = wasm.load_wasm("python") -- AOT，约 1.4 秒从磁盘缓存加载
py.init()
py.exec("import time as t")
print(py.eval("t.time()"))
```

注意：使用 stdio（`load_wasm(..., true)`）时，python 的 `sys.stderr.write` 是行缓冲
（chicory 报告 isatty），没有换行时需显式 `flush()`；`print` 自带换行会自动刷。

# 测试

测试脚本在 `wasm/test/`（dev 运行目录），入口 `test.lua`
（在电脑里 `dofile("wasm/test/test.lua")` 运行）。结果写入 `shared/`。
