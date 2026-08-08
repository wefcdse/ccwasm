-- AOT 加载测试：验证 useAoT=true 加载 + AOT 磁盘缓存
-- 首次运行：AOT 编译并写入 .minecraft/wasm_cache/（wasm 内容 sha256 分桶 jar）
-- 再次运行：命中缓存，加载应明显变快
-- 结果写入 shared/aot_test.txt
local clear_f = fs.open("shared/aot_test.txt", "w")
if clear_f then
    clear_f.close()
end

local function record(msg)
    print(msg)
    local f = fs.open("shared/aot_test.txt", "a")
    if f then
        f.writeLine(msg)
        f.close()
    end
end

local t0 = os.clock()
local m = wasm.load_wasm("type_test", true)
local t1 = os.clock()
record(string.format("ok: AOT load type_test, elapsed %.3fs", t1 - t0))
m.init()

local function check(cond, msg, detail)
    if cond then
        record("ok: " .. msg)
    else
        record("FAIL: " .. msg .. " -> " .. tostring(detail))
    end
end

check(m.version() ~= nil, "AOT version() callable", m.version())
check(m.pi32(42) == 42, "AOT i32 round-trip", m.pi32(42))
check(m.pstr("hello") == "hello", "AOT string round-trip", m.pstr("hello"))
record("aot_test: done")
print("result saved to shared/aot_test.txt")
