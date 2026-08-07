-- load_wasm 加载顺序验证（global / save / auto）
-- 前提：全局 wasm/type_test.wasm 为普通构建（position()="global"）
--       存档 computercraft/wasm/type_test.wasm 为 --features from_save 构建（position()="save"）
-- 结果写入 shared/load_order.txt
local clear_f = fs.open("shared/load_order.txt", "w")
if clear_f then
    clear_f.close()
else
    print("WARN: cannot open shared/load_order.txt for write")
end

local failed = 0

local function record(msg)
    print(msg)
    local f = fs.open("shared/load_order.txt", "a")
    if f then
        f.writeLine(msg)
        f.close()
    else
        print("WARN: cannot write shared/load_order.txt")
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

local function load(source)
    local ok, m = pcall(wasm.load_wasm, "type_test", false, source)
    if ok then
        return true, m.position()
    else
        return false, tostring(m)
    end
end

-- 1. 三个来源各自加载，验证 position
local ok, g = load("global")
check(ok and g == "global", "global -> global build", g)
local ok2, s = load("save")
check(ok2 and s == "save", "save -> save build", s)
local ok3, a = load("auto")
check(ok3 and a == "save", "auto prefers save build", a)
local ok4, d = load() -- 不传第三参数 = auto
check(ok4 and d == "save", "no arg defaults to auto (save first)", d)

-- 2. 存档没有的程序，auto 应回落到全局
local ok5, m5 = pcall(wasm.load_wasm, "obj_demo", false, "auto")
check(ok5 and m5.pass ~= nil, "auto falls back to global when save missing", tostring(m5))

-- 3. save 强制：存档没有则报错
local ok6, err6 = pcall(wasm.load_wasm, "obj_demo", false, "save")
check(not ok6 and tostring(err6):find("not found in save dir") ~= nil, "save errors when missing", tostring(err6))

-- 4. global 强制：即使存档有同名，也加载全局版
local ok7, g7 = load("global")
check(ok7 and g7 == "global", "global ignores save build", g7)

if failed == 0 then
    record("load_order: all tests passed")
else
    record("load_order: " .. failed .. " test(s) failed")
end
print("result saved to shared/load_order.txt")
