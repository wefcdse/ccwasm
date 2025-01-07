package com.iung.ccwasm.wasm_api;

import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.wasm.types.Value;

@FunctionalInterface
public interface ValuedHostFuncHandler {
    Value[] apply(Instance instance, Value... args);
}

