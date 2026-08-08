-- exec_wasm_aot: 后台预编译 + AOT 磁盘缓存加载 wasm，然后驱动协程循环
-- 用法: exec_wasm_aot <wasm名> [args...]
-- 首次运行会后台编译（不阻塞电脑线程，避免超时被杀）并写磁盘缓存；
-- 之后命中缓存秒加载。也可先用 wasm_compile_aot 预热。
Args = { ... }
wasm_name = Args[1]

_G.args = {}
for index, value in ipairs(Args) do
    if index > 1 then
        args[index - 1] = value
    end
end

_G.global = _G

print("precompiling " .. wasm_name .. " ...")
wasm.precompile(wasm_name)
while wasm.precompile_done(wasm_name) == "compiling" do
    os.sleep(0.2)
end
if wasm.precompile_done(wasm_name) ~= "done" then
    error("precompile failed: " .. wasm_name)
end

local t0 = os.clock()
_G.wasm_mod = wasm.load_wasm(wasm_name, true)
print(string.format("loaded %s in %.1fs", wasm_name, os.clock() - t0))
wasm_mod.init()

while true do
    wasm_mod.tick()
    if wasm_mod.eval_string() ~= nil then
        wasm_mod.eval_result(load(wasm_mod.eval_string())())
    end
    if wasm_mod.stopped() then
        break
    end
end
