-- stdio 测试：useStdio=true 加载模块，验证 stdin/stdout/stderr 三个方法
-- load_wasm 第 4 个参数 true 启用；默认不启用（无这三个方法）
-- 结果写入 shared/stdio_test.txt
local clear_f = fs.open("shared/stdio_test.txt", "w")
if clear_f then
    clear_f.close()
end

local failed = 0

local function record(msg)
    print(msg)
    local f = fs.open("shared/stdio_test.txt", "a")
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
        record("FAIL: " .. msg .. " -> " .. tostring(detail))
    end
end

-- 默认不启用：无 stdin/stdout/stderr 方法
local m0 = wasm.load_wasm("type_test", false)
check(m0.stdin == nil and m0.stdout == nil and m0.stderr == nil,
    "default module has no stdio methods", "m0.stdin=" .. tostring(m0.stdin))

-- 启用 stdio 加载 python（先预编译，避免同步编译超时）
wasm.precompile("python")
while wasm.precompile_done("python") == "compiling" do
    os.sleep(0.2)
end
if wasm.precompile_done("python") ~= "done" then
    record("FAIL: precompile python -> " .. tostring(wasm.precompile_done("python")))
    record("stdio_test: done")
    return
end
local m = wasm.load_wasm("python", true, "auto", true)
check(m.stdin ~= nil and m.stdout ~= nil and m.stderr ~= nil,
    "stdio module has stdin/stdout/stderr methods", "")

m.init()
m.exec("print('hello from wasm')")
local out = m.stdout()
check(out == "hello from wasm\n", "stdout captures print", tostring(out))

local out2 = m.stdout()
check(out2 == "", "stdout cleared after read", tostring(out2))

-- stdin
m.stdin("abc\n")
local ok, r = pcall(m.eval, "input()")
if ok then
    check(r == "abc", "stdin feeds input()", tostring(r))
else
    check(false, "stdin feeds input()", "exception: " .. tostring(r))
end

-- stderr
local ok2, e2 = pcall(function()
    m.exec("import sys; sys.stderr.write('oops'); sys.stderr.flush()")
end)
if ok2 then
    check(m.stderr() == "oops", "stderr captures sys.stderr.write", tostring(m.stderr()))
else
    check(false, "stderr capture", "exception: " .. tostring(e2))
end

if failed == 0 then
    record("stdio_test: all tests passed")
else
    record("stdio_test: " .. failed .. " test(s) failed")
end
print("result saved to shared/stdio_test.txt")
