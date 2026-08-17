-- 测试总入口：依次执行全部测试脚本
-- 每个脚本会把自己的结果写到 shared/ 下对应文件
-- 本脚本汇总每个脚本的执行状态到 shared/all_tests.txt
local clear_f = fs.open("shared/all_tests.txt", "w")
if clear_f then
    clear_f.close()
end

local failed = 0

local function record(msg)
    print(msg)
    local f = fs.open("shared/all_tests.txt", "a")
    if f then
        f.writeLine(msg)
        f.close()
    end
end

local tests = {
    "wasm/test/obj_demo.lua",
    "wasm/test/test_basic.lua",
    "wasm/test/test_bin.lua",
    "wasm/test/load_order.lua",
    "wasm/test/test_aot.lua",
    "wasm/test/test_python.lua",
    "wasm/test/test_stdio.lua",
    "wasm/test/test_echo.lua",
}

for _, path in ipairs(tests) do
    local ok, err = pcall(dofile, path)
    if ok then
        record("ok: " .. path)
    else
        failed = failed + 1
        record("FAIL: " .. path .. " -> " .. tostring(err))
    end
end

if failed == 0 then
    record("all_tests: all scripts ran")
else
    record("all_tests: " .. failed .. " script(s) failed")
end
print("result saved to shared/all_tests.txt")
