-- type_test 全面类型调用测试
-- 结果实时写入 shared/test.txt（可读写共享文件夹，跟随存档）
-- 所有调用都用 pcall 包装：调用异常也记录原因，不中断，跑完所有用例
local m = wasm.load_wasm("type_test", false)
m.init()

local clear_f = fs.open("shared/test.txt", "w")
if clear_f then
    clear_f.close()
else
    print("WARN: cannot open shared/test.txt for write")
end

local failed = 0

local function record(msg)
    print(msg)
    local f = fs.open("shared/test.txt", "a")
    if f then
        f.writeLine(msg)
        f.close()
    else
        print("WARN: cannot write shared/test.txt")
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

record("type_test version: " .. m.version())

-- 通用用例：pcall 包装，返回 ok + 值；checkfn(r) -> pass, info
local function t(name, f, checkfn)
    local ok, r = pcall(f)
    if ok then
        local pass, info = checkfn(r)
        check(pass, name, info)
    else
        check(false, name, "exception: " .. tostring(r))
    end
end

record("== int ==")
t("i32 round-trip", function() return m.pi32(42) end, function(r) return r == 42, tostring(r) end)
t("i32 negative", function() return m.pi32(-7) end, function(r) return r == -7, tostring(r) end)
t("i64 round-trip", function() return m.pi64(2^40) end, function(r) return r == 2^40, tostring(r) end)
t("i64 negative", function() return m.pi64(-2^40) end, function(r) return r == -2^40, tostring(r) end)
t("i64 export", function() return m.i64out() end, function(r) return r == 2^40, tostring(r) end)

record("== float ==")
t("f64 round-trip", function() return m.pf64(2.25) end, function(r) return math.abs(r - 2.25) < 1e-12, tostring(r) end)
t("f64 negative", function() return m.pf64(-0.5) end, function(r) return math.abs(r - (-0.5)) < 1e-12, tostring(r) end)
t("f64 export", function() return m.f64out() end, function(r) return math.abs(r - 2.25) < 1e-12, tostring(r) end)
t("f32 export", function() return m.f32out() end, function(r) return math.abs(r - 1.5) < 1e-6, tostring(r) end)

record("== string ==")
t("string ascii", function() return m.pstr("hello") end, function(r) return r == "hello", tostring(r) end)
t("string empty", function() return m.pstr("") end, function(r) return r == "", tostring(r) end)
t("string escapes", function() return m.pstr("a\nb\t\"c'") end, function(r) return r == "a\nb\t\"c'", tostring(r) end)
record("skip: string utf8 (known limitation: cc lua string 不支持 utf8)")
t("string long 10k", function() return m.pstr(string.rep("x", 10000)) end, function(r) return r == string.rep("x", 10000), "len=" .. tostring(#r) end)

record("== bool ==")
t("bool true", function() return m.pbool(true) end, function(r) return r == true, tostring(r) end)
t("bool false", function() return m.pbool(false) end, function(r) return r == false, tostring(r) end)

record("== nil / option ==")
t("option none", function() return m.pnil(nil) end, function(r) return r == nil, tostring(r) end)
t("option some", function() return m.pnil(7) end, function(r) return r == 7, tostring(r) end)
t("option some negative", function() return m.pnil(-1) end, function(r) return r == -1, tostring(r) end)

record("== number ==")
t("number int", function() return m.pnum(3) end, function(r) return r == 3, tostring(r) end)
t("number float", function() return m.pnum(3.5) end, function(r) return r == 3.5, tostring(r) end)
t("number negative", function() return m.pnum(-2.25) end, function(r) return r == -2.25, tostring(r) end)
t("number zero", function() return m.pnum(0) end, function(r) return r == 0, tostring(r) end)

record("== tuple / multi return ==")
local ok, a, b, c = pcall(m.ptuple, 1, "x", true)
if ok then
    check(a == 1 and b == "x" and c == true, "tuple multi-param multi-return", tostring(a) .. "," .. tostring(b) .. "," .. tostring(c))
else
    check(false, "tuple multi-param multi-return", "exception: " .. tostring(a))
end
local ok2, i, s = pcall(m.f_multi)
if ok2 then
    check(i == 42 and s == "multi", "two return values", tostring(i) .. "," .. tostring(s))
else
    check(false, "two return values", "exception: " .. tostring(i))
end

record("== vec ==")
t("vec empty", function() return m.pvec() end, function(r) return r == 0, tostring(r) end)
t("vec single", function() return m.pvec(1) end, function(r) return r == 1, tostring(r) end)
t("vec sum 3", function() return m.pvec(1, 2, 3) end, function(r) return r == 6, tostring(r) end)
t("vec sum 5", function() return m.pvec(1, 2, 3, 4, 5) end, function(r) return r == 15, tostring(r) end)

record("== either ==")
t("either str", function() return m.peither("abc") end, function(r) return r == "str:abc", tostring(r) end)
t("either int", function() return m.peither(9) end, function(r) return r == "num:9", tostring(r) end)
t("either float", function() return m.peither(2.5) end, function(r) return r == "num:2.5", tostring(r) end)

record("== lua object ==")
local obj = { name = "test" }
t("obj pass round-trip", function() return m.pobj(obj) end, function(r) return type(r) == "table" and r.name == "test", tostring(r) end)
t("obj hold/give", function() m.obj_hold(obj) return m.obj_give() end, function(r) return type(r) == "table" and r.name == "test", tostring(r) end)
t("obj empty after take", function() return m.obj_give() end, function(r) return r == nil, tostring(r) end)

record("== error propagation ==")
local ok3, err = pcall(m.f_err)
check(not ok3, "error raises lua exception", tostring(err))
check(tostring(err):find("test error") ~= nil, "error message preserved", tostring(err))

record("== eval ==")
m.eval_test()
local got = nil
for i = 1, 500 do
    local es = m.eval_string()
    if es ~= nil then
        m.eval_result(load(es)())
    end
    m.tick()
    got = m.eval_get()
    if got ~= nil or m.stopped() then
        break
    end
end
check(got == 3, "eval returns 1+2", tostring(got))

if failed == 0 then
    record("type_test: all tests passed")
else
    record("type_test: " .. failed .. " test(s) failed")
end
print("result saved to shared/test.txt")
