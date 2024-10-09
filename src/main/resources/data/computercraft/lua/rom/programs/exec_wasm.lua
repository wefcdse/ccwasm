Args = { ... }
wasm_name = Args[1]
-- print(wasm_name)

_G.global = _G
_G.wasm_mod = wasm.load_wasm(wasm_name)
_G.args = Args
wasm_mod.init()

while true do
    wasm_mod.tick()
    if wasm_mod.eval_string() ~= nil then
        wasm_mod.eval_result(load(wasm_mod.eval_string())())
    end
end
