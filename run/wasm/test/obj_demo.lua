-- obj_demo 用法演示：把 Lua table 传进 wasm 再原样取回
-- 测试结果实时写入 shared/obj_demo.txt（可读写共享文件夹，跟随存档）
-- 不用 assert：失败记录到文件+命令行，不中断，跑完所有用例
-- 注意：CC 每次 lua<->java 转换都会生成新的 table 引用，不能比 == 引用，只能比内容
local m = wasm.load_wasm("obj_demo", false)

local clear_f = fs.open("shared/obj_demo.txt", "w")
if clear_f then
    clear_f.close()
else
    print("WARN: cannot open shared/obj_demo.txt for write")
end

local failed = 0

local function record(msg)
    print(msg)
    local f = fs.open("shared/obj_demo.txt", "a")
    if f then
        f.writeLine(msg)
        f.close()
    else
        print("WARN: cannot write shared/obj_demo.txt")
    end
end

local function check(cond, msg)
    if cond then
        record("ok: " .. msg)
    else
        failed = failed + 1
        record("FAIL: " .. msg)
    end
end

-- 1. pass: 原样往返，验证 SlotMap key 正确（旧 bug 是全部挤在 key=0）
local t1 = { name = "one", n = 1 }
local t2 = { name = "two", n = 2 }
local t3 = { name = "three", n = 3 }
local r1 = m.pass(t1)
local r2 = m.pass(t2)
local r3 = m.pass(t3)
check(type(r1) == "table" and r1.name == "one", "pass t1 round-trip")
check(type(r2) == "table" and r2.name == "two", "pass t2 round-trip")
check(type(r3) == "table" and r3.name == "three", "pass t3 round-trip")

-- 2. hold/give: wasm 侧保存，再取回；hold 覆盖旧对象时旧 slot 会被释放
m.hold(t1)
local g1 = m.give()
check(type(g1) == "table" and g1.name == "one", "give t1 back")
check(m.give() == nil, "give empty after take")

m.hold(t1)
m.hold(t2) -- 覆盖 t1，旧 t1 的 slot 被释放（日志会打 previous object released）
local g2 = m.give()
check(type(g2) == "table" and g2.name == "two", "hold replace: give latest held")
check(m.give() == nil, "give empty after take (only one slot)")

-- 3. drop_held: 释放 slot 后 give 返回空
m.hold(t3)
m.drop_held()
check(m.give() == nil, "drop_held frees slot")

-- 4. 多对象同时存活（不同 key 不串数据）
local t4 = { name = "four", n = 4 }
local t5 = { name = "five", n = 5 }
m.hold(t4)
m.hold(t5)
local r4 = m.pass(t4)
local r5 = m.pass(t5)
check(type(r4) == "table" and r4.name == "four", "pass t4 while t5 held")
check(type(r5) == "table" and r5.name == "five", "pass t5 while t4 held")
local g5 = m.give()
check(type(g5) == "table" and g5.name == "five", "held t5 not corrupted")

if failed == 0 then
    record("obj_demo: all tests passed")
else
    record("obj_demo: " .. failed .. " test(s) failed")
end
print("result saved to shared/obj_demo.txt")
