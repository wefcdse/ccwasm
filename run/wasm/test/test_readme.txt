ccwasm 测试文件说明
===================

本目录（run/wasm/test/）存放 ccwasm 的测试脚本。
对应 wasm 测试模块源码在 ccwasm/examples/ 下（obj_demo、type_test）。

一、编译 wasm 测试模块
----------------------

1. obj_demo（LuaObject 对象往返测试）：
   cd ccwasm/examples/obj_demo
   cargo build -r --lib --target wasm32-wasip1
   产物：examples/obj_demo/target/wasm32-wasip1/release/obj_demo.wasm

2. type_test（全面类型测试 + 加载顺序验证）：
   cd ccwasm/examples/type_test
   cargo build -r --lib --target wasm32-wasip1
   cargo build -r --lib --target wasm32-wasip1 --features from_save

   注意：两次构建会覆盖同一个产物文件，必须分别拷贝：
   - 普通构建  -> 拷到全局目录（position() 返回 "global"）
   - from_save -> 拷到存档目录（position() 返回 "save"）

二、放置位置
------------

- 全局 wasm 目录（所有存档共享）：
  run/wasm/<名字>.wasm
  游戏中挂载为 wasm/

- 存档专属 wasm 目录（跟随存档，服务器启动时自动创建）：
  run/saves/<存档名>/computercraft/wasm/<名字>.wasm
  游戏中挂载为 wasm_save/

- load_wasm 加载顺序（第三参数）：
  "auto"   默认，优先存档目录，缺失则回落全局
  "global" 强制全局目录
  "save"   强制存档目录，缺失报错

  加载顺序验证前提：全局放普通构建，存档放 --features from_save 构建。

三、测试脚本与输出
------------------

游戏内（电脑终端）运行：
  dofile("wasm/test/obj_demo.lua")    -- LuaObject 往返测试
  dofile("wasm/test/test.lua")        -- 全面类型测试
  dofile("wasm/test/load_order.lua")  -- 加载顺序验证

输出文件（游戏内路径 shared/，磁盘路径 run/saves/<存档名>/computercraft/shared/）：
  shared/obj_demo.txt     -- obj_demo 测试结果
  shared/test.txt         -- type_test 测试结果（首行是 type_test version: <版本>）
  shared/load_order.txt   -- 加载顺序验证结果

结果格式：每行一条用例，ok: 通过 / FAIL: 失败（含原因）/ skip: 跳过（已知限制），
末尾一行是汇总（all tests passed 或 N test(s) failed）。

四、已知限制
------------

- 字符串通道不支持 UTF-8 多字节字符（中文等），测试中标记为 skip。
- 数字参数：Lua 数字一律以 f64 传入，Rust 侧 i32/i64/f32/f64 的 import
  会做数值适配（整数检查 + 范围检查）。
