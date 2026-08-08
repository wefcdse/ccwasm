-- wasm_compile_aot: 后台预编译 wasm 到 AOT 磁盘缓存
-- 用法: wasm_compile_aot <wasm名> [global|save|auto]
-- 编译在后台线程进行（不占电脑线程，避免超时被杀），完成后写 .minecraft/wasm_cache/
-- 之后 wasm.load_wasm(名字, true) / exec_wasm_aot 命中缓存秒加载
Args = { ... }
local wasm_name = Args[1]
if not wasm_name then
    print("usage: wasm_compile_aot <wasm名> [global|save|auto]")
    return
end
local source = Args[2]

print("precompiling " .. wasm_name .. " ...")
wasm.precompile(wasm_name, source)
local t0 = os.clock()
local status = nil
while true do
    status = wasm.precompile_done(wasm_name)
    if status ~= "compiling" then
        break
    end
    os.sleep(0.2)
end
print("precompile " .. wasm_name .. ": " .. tostring(status) .. " (" .. math.floor(os.clock() - t0) .. "s)")
