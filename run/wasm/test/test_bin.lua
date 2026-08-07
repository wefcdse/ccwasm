-- 二进制传输测试：Lua 读取文件字节传给 Rust，Rust 与编译时嵌入的 bin_test.txt 比对
-- 数据：0x00-0xFF 全字节 + UTF-8 中文文本（322 字节）
-- 结果写入 shared/bin_test.txt
local clear_f = fs.open("shared/bin_test.txt", "w")
if clear_f then
    clear_f.close()
end

local failed = 0

local function record(msg)
    print(msg)
    local f = fs.open("shared/bin_test.txt", "a")
    if f then
        f.writeLine(msg)
        f.close()
    end
end

local function check(cond, msg, detail)
    if cond then
        record("ok: " .. msg)
    else
        failed = failed + 1
        if detail then
            record("FAIL: " .. msg .. " -> " .. detail)
        else
            record("FAIL: " .. msg)
        end
    end
end

local f = fs.open("wasm/test/bin_test.txt", "rb")
local data = nil
if f then
    data = f.readAll()
    f.close()
else
    record("FAIL: cannot open wasm/test/bin_test.txt")
end

check(data ~= nil, "lua reads file bytes", tostring(data and #data))

if data then
    local m = wasm.load_wasm("type_test", false, "global")
    local r = m.check_bin(data)
    if r == "match" then
        record("ok: binary round-trip exact match (" .. #data .. " bytes)")
    else
        failed = failed + 1
        record("FAIL: " .. r)
    end

    -- 对照 1：中文从文件字节取（后 66 字节 UTF-8），经 pstr 往返
    local tail = string.sub(data, 257)
    local r1 = m.pstr(tail)
    check(r1 == tail, "utf8 from file bytes via pstr round-trip", tostring(#r1) .. " vs " .. tostring(#tail))

    -- 对照 2：中文源码字面量，经 pstr 往返（复现旧 FAIL）
    local lit = "中文测试"
    local r2 = m.pstr(lit)
    check(r2 == lit, "utf8 source literal via pstr round-trip", tostring(#r2) .. " vs " .. tostring(#lit))
end

if failed == 0 then
    record("bin_test: all tests passed")
else
    record("bin_test: " .. failed .. " test(s) failed")
end
print("result saved to shared/bin_test.txt")
