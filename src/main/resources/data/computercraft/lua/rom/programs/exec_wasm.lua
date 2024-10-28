Args = { ... }
wasm_name = Args[1]
-- print(wasm_name)

_G.args = {}
for index, value in ipairs(Args) do
    if index > 1 then
        args[index - 1] = value
    end
end


_G.global = _G
_G.wasm_mod = wasm.load_wasm(wasm_name)
wasm_mod.init()

while true do
    wasm_mod.tick()
    if wasm_mod.eval_string() ~= nil then
        wasm_mod.eval_result(load(wasm_mod.eval_string())())
    end
    if wasm_mod.stopped() then
        break
    end
end
