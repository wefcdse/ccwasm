package com.iung.ccwasm;

import com.dylibso.chicory.runtime.ExportFunction;
import com.dylibso.chicory.runtime.HostImports;
import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.runtime.Module;
import com.iung.ccwasm.wasm_api.HostFuncs;
import com.iung.ccwasm.wasm_api.IOHandler;
import com.iung.ccwasm.wasm_api.IOValue;

import java.io.File;

public class Main {
    public static void main(String[] _args) {
        IOHandler io = new IOHandler();
        HostFuncs hfs = new HostFuncs(io);

        HostImports hi = HostImports.builder()
//                .addFunction(HostFunc.show_str())
                .addFunction(hfs.all())
                .addFunction(hfs.wasi())
                .build();
        io.push(IOValue.of("import time\ntime.time()"));
//        io.push(IOValue.of(32));
//        io.push(IOValue.of(123123L));
//        io.push(IOValue.of("from java 2"));
//        io.push(IOValue.of(1.61f));
//        io.push(IOValue.of(1.231d));
//        io.push(IOValue.type(0));


        Module module = Module.builder(new File("./a.wasm")).withHostImports(hi).build();
        Instance instance = module.instantiate();
        ExportFunction a = instance.export("entry");
        a.apply();

        System.out.println(io.pop().asString());
//        System.out.println(io.pop().asInt());
//        System.out.println(io.pop().asLong());
//        System.out.println(io.pop().asFloat());
//        System.out.println(io.pop().asDouble());
    }
}
