package com.iung.ccwasm;

import com.dylibso.chicory.runtime.*;
import com.dylibso.chicory.runtime.Module;
import com.dylibso.chicory.wasm.types.Value;
import com.dylibso.chicory.wasm.types.ValueType;
import com.iung.ccwasm.wasm_api.HostFuncs;
import com.iung.ccwasm.wasm_api.IOHandler;
import com.iung.ccwasm.wasm_api.IOValue;

import java.io.File;
import java.util.List;

public class Main {
    public static void main(String[] _args) {
        IOHandler io = new IOHandler();
        HostFuncs hfs = new HostFuncs(io);

        HostImports hi = HostImports.builder()
//                .addFunction(HostFunc.show_str())
                .addFunction(hfs.wasi())
                .build();
        io.to_wasm_push(IOValue.of("import time\ntime.time()"));
//        io.push(IOValue.of(32));
//        io.push(IOValue.of(123123L));
//        io.push(IOValue.of("from java 2"));
//        io.push(IOValue.of(1.61f));
//        io.push(IOValue.of(1.231d));
//        io.push(IOValue.type(0));


        Module module = Module.builder(new File("./a.wasm")).withHostImports(hi).build();
        Instance instance = module.instantiate();
        ExportFunction a = instance.export("call");
        a.apply();
        System.out.println( a.apply()[0].asInt());
        a.apply();
        System.out.println( a.apply()[0].asInt());
        a.apply();
        System.out.println( a.apply()[0].asInt());
        a.apply();
        System.out.println( a.apply()[0].asInt());
        a.apply();
        System.out.println( a.apply()[0].asInt());
        a.apply();
        System.out.println( a.apply()[0].asInt());
        a.apply();
        System.out.println( a.apply()[0].asInt());

//        System.out.println(io.pop().asInt());
//        System.out.println(io.pop().asLong());
//        System.out.println(io.pop().asFloat());
//        System.out.println(io.pop().asDouble());
    }
}
