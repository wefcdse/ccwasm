Args = { ... }
Wasm_name = Args[1]
Side = Args[2]
if Side == nil then
    Side = "front"
end
if Side ~= "front" and Side ~= "back" and Side ~= "left" and Side ~= "right" and Side ~= "top" and Side ~= "bottom" then
    Side = "front"
end
Edge = Args[3]
if Edge == nil then
    Edge = "posedge"
end
if Edge ~= "posedge" and Edge ~= "negedge" and Edge ~= "posneg" then
    Edge = "posedge"
end

-- print(Wasm_name)
-- print(Side)

_G.global = _G
_G.wasm_mod = wasm.load_wasm(Wasm_name)

_G.args = {}
for index, value in ipairs(Args) do
    if index > 3 then
        args[index - 3] = value
    end
end
wasm_mod.init()


LastTickInput = redstone.getAnalogInput(Side)
function HandleReload()
    local if_reload = false;
    local thisTick = redstone.getAnalogInput(Side)
    if (Edge == "posedge" or Edge == "posneg") and thisTick > LastTickInput then
        if_reload = true
    end
    if (Edge == "negedge" or Edge == "posneg") and thisTick < LastTickInput then
        if_reload = true
    end

    LastTickInput = thisTick
    return if_reload
end

while true do
    if HandleReload() then
        wasm_mod = wasm.load_wasm(Wasm_name)
        wasm_mod.init()
    end
    wasm_mod.tick()
    if wasm_mod.eval_string() ~= nil then
        wasm_mod.eval_result(load(wasm_mod.eval_string())())
    end
end
