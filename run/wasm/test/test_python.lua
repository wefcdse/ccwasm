-- AOT 大模块测试：python.wasm（9.6MB）用 AOT 加载
-- 1.7.5 修复 ClassTooLarge 崩溃：大模块 AOT 不再崩（bridge 类拆分）
-- 首次编译用后台预编译（wasm.precompile，不占电脑线程，避免 CC 超时杀编译），完成后写磁盘缓存
-- 再连续 load_wasm 三次，对比每次耗时（第 1 次命中缓存 vs 之后也应秒开）
-- 结果写入 shared/python_aot.txt
local clear_f = fs.open("shared/python_aot.txt", "w")
if clear_f then
    clear_f.close()
end

local function record(msg)
    print(msg)
    local f = fs.open("shared/python_aot.txt", "a")
    if f then
        f.writeLine(msg)
        f.close()
    end
end

local pre = os.clock()
wasm.precompile("python")
local status = nil
while true do
    status = wasm.precompile_done("python")
    if status ~= "compiling" then
        break
    end
    os.sleep(0.2)
end
record(string.format("precompile status: %s (%.0fs)", status, os.clock() - pre))
if status ~= "done" then
    record("python_aot: done")
    return
end

local elapsed = {}
local m = nil
local failed = 0
local function check(cond, msg, detail)
    if cond then
        record("ok: " .. msg)
    else
        failed = failed + 1
        record("FAIL: " .. msg .. " -> " .. tostring(detail))
    end
end

for i = 1, 3 do
    local t0 = os.clock()
    local ok, inst = pcall(wasm.load_wasm, "python", true)
    local t1 = os.clock()
    if not ok then
        record("FAIL: AOT load python #" .. i .. " -> " .. tostring(inst))
        record("python_aot: done")
        return
    end
    m = inst
    elapsed[i] = t1 - t0
    record(string.format("ok: AOT load python #%d, elapsed %.1fs", i, elapsed[i]))

    local ok2, r = pcall(function()
        m.init()
        return m.eval("1 + 2")
    end)
    if ok2 then
        check(tonumber(r) == 3, "use #" .. i .. " eval 1+2=3 (type " .. type(r) .. ")", tostring(r))
    else
        check(false, "use #" .. i .. " eval", "exception: " .. tostring(r))
    end
end
record(string.format("load times: %.1fs / %.1fs / %.1fs", elapsed[1], elapsed[2], elapsed[3]))

local ok, e = pcall(function()
    m.init()
    m.exec("x = 1 + 2")
end)
check(ok, "python init + exec", e)

local ok2, r = pcall(m.eval, "x + 1")
if ok2 then
    check(tonumber(r) == 4, "python eval x+1=4 (type " .. type(r) .. ")", tostring(r))
else
    check(false, "python eval x+1", "exception: " .. tostring(r))
end

if failed == 0 then
    record("python_aot: all tests passed")
else
    record("python_aot: " .. failed .. " test(s) failed")
end
print("result saved to shared/python_aot.txt")
