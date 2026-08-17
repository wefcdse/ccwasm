-- 全类型 echo 测试：echo_all 每个类型收一个再原样吐回，验证 import+export 双向无损
-- 类型：i32 / i64 / f32 / f64 / bool / String / Vec<u8> / Option<i32> / Number
-- 结果写入 shared/echo_test.txt
local clear_f = fs.open("shared/echo_test.txt", "w")
if clear_f then
    clear_f.close()
end

local failed = 0

local function record(msg)
    print(msg)
    local f = fs.open("shared/echo_test.txt", "a")
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

local m = wasm.load_wasm("type_test", false, "global")

local a, b, c, d, e, f, g, h, i = m.echo_all(
    42,            -- i32
    2 ^ 40,        -- i64
    1.5,           -- f32（可精确表示，round-trip 无损）
    2.25,          -- f64
    true,          -- bool
    "中文abc",     -- String（UTF-8 字节）
    "你好，世界",  -- Vec<u8>（原样字节）
    7,             -- Option<i32> some
    3.5            -- Number
)
check(a == 42, "echo i32", tostring(a))
check(b == 2 ^ 40, "echo i64", tostring(b))
check(c == 1.5, "echo f32", tostring(c))
check(d == 2.25, "echo f64", tostring(d))
check(e == true, "echo bool", tostring(e))
check(f == "中文abc", "echo String (chinese)", tostring(#f) .. " bytes")
check(g == "你好，世界", "echo Vec<u8> (chinese bytes)", tostring(#g) .. " bytes")
check(h == 7, "echo Option some", tostring(h))
check(i == 3.5, "echo Number", tostring(i))

-- Option none：None 不导出 Lua 值（空），用单参函数测（末尾 → nil）
-- （echo_all 里 Option 夹在中间，None 会让后续 Number 前移，属多返回值布局行为）
local on = m.pnil(nil)
check(on == nil, "echo Option none", "on=" .. tostring(on))

if failed == 0 then
    record("echo_test: all tests passed")
else
    record("echo_test: " .. failed .. " test(s) failed")
end
print("result saved to shared/echo_test.txt")
