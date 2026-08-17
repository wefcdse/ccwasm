Args = { ... }
wasm_name = Args[1]
-- print(wasm_name)

_G.args = {}
for index, value in ipairs(Args) do
    if index > 1 then
        args[index - 1] = value
    end
end


-- print("precompiling " .. wasm_name .. " ...")
wasm.precompile(wasm_name)
while wasm.precompile_done(wasm_name) == "compiling" do
    os.sleep(0.0)
end
if wasm.precompile_done(wasm_name) ~= "done" then
    error("precompile failed: " .. wasm_name)
end


_G.global = _G
_G.wasm_mod = wasm.load_wasm(wasm_name, true, "auto", true)
wasm_mod.init()

while true do
    wasm_mod.tick()
    if wasm_mod.eval_string() ~= nil then
        wasm_mod.eval_result(load(wasm_mod.eval_string())())
    end

    stdout = wasm_mod.stdout()
    stderr = wasm_mod.stderr()
    if stdout ~= "" then
        io.write(stdout)
    end
    if stderr ~= "" then
        io.write("[STDERR]: "..stderr.."\n")
    end
    if wasm_mod.stopped() then
        break
    end
end
