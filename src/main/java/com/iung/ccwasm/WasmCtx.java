package com.iung.ccwasm;

import com.dylibso.chicory.experimental.aot.AotMachine;
import com.dylibso.chicory.runtime.*;
//import com.dylibso.chicory.runtime.
import com.dylibso.chicory.wasm.Parser;
import com.dylibso.chicory.wasm.WasmModule;
import com.iung.ccwasm.wasm_api.HostFuncs;
import com.iung.ccwasm.wasm_api.IOHandler;
import com.iung.ccwasm.wasm_api.IOValue;
import dan200.computercraft.api.lua.*;

import java.io.File;
import java.nio.ByteBuffer;
import java.util.Objects;

public class WasmCtx implements IDynamicLuaObject {
    static String[] METHODS = {"load_wasm", "run_func"};
    IOHandler ioHandler;
    //    Module wasm_module;
    Instance wasm_instance;
    String[] methods;

    public WasmCtx(File file) {

        IOHandler io = new IOHandler();
        HostFuncs hfs = new HostFuncs(io);

        Store store = new Store();
        WasmModule wasmModule = Parser.parse(file);

        store.addFunction(hfs.wasi());
        store.addFunction(hfs.all());
        store.addFunction(HostFuncs.show_str());
//        HostImports hi = HostImports.builder()
//                .addFunction(HostFuncs.show_str())
//                .addFunction(hfs.all())
//                .addFunction(hfs.wasi())
//                .build();

        this.ioHandler = io;
//        this.wasm_module = Module.builder(file).withMachineFactory(AotMachine::new).withHostImports(hi).build();
//        this.wasm_instance = store.instantiate("aaa",wasmModule);
        this.wasm_instance = Instance
                .builder(wasmModule)
                .withImportValues(store.toImportValues())
                .withMachineFactory(AotMachine::new)
                .build();

        ExportFunction a = this.wasm_instance.export("export_func");
        a.apply();
        this.methods = this.ioHandler.getFrom_wasm().stream().map(IOValue::asString).toArray(String[]::new);
        this.ioHandler.clear_all();
    }

    @Override
    public String[] getMethodNames() {
        return methods;
    }

    @Override
    public MethodResult callMethod(ILuaContext context, int method, IArguments arguments) throws LuaException {
//        Ccwasm.LOGGER.info("wasm ctx: Slots {}", ioHandler.obj_hold.count());
//        Ccwasm.LOGGER.info(arguments.getType(0));

        try {
            var func = methods[method];
            if (func.equals("eval_result")) {
                this.ioHandler.setTo_eval(null);
                for (int i = 0; i < arguments.count(); i++) {
//                    Ccwasm.LOGGER.info(arguments.getType(i));
                    if (arguments.getType(i).equals("nil")) {
                        ioHandler.to_eval_push(new IOValue(IOValue.Nil, null));
                    } else if (arguments.getType(i).equals("string")) {
                        ByteBuffer buf = arguments.getBytes(i);
                        byte[] arr = new byte[buf.remaining()];
                        buf.get(arr);
                        ioHandler.to_eval_push(IOValue.of_obj(arr));
                    } else {
                        ioHandler.to_eval_push(IOValue.of_obj(arguments.get(i)));
                    }
                }
                return MethodResult.of();
            }
            if (func.equals("eval_string")) {
                return MethodResult.of(ioHandler.getTo_eval());
            }
            this.ioHandler.clear_all();
            for (int i = 0; i < arguments.count(); i++) {
                if (arguments.getType(i).equals("nil")) {
                    ioHandler.to_wasm_push(new IOValue(IOValue.Nil, null));
                } else if (arguments.getType(i).equals("string")) {
                    ByteBuffer buf = arguments.getBytes(i);
                    byte[] arr = new byte[buf.remaining()];
                    buf.get(arr);
                    ioHandler.to_eval_push(IOValue.of_obj(arr));
                } else {
                    ioHandler.to_eval_push(IOValue.of_obj(arguments.get(i)));
                }
            }


            ExportFunction a = this.wasm_instance.export(func);
            a.apply();
            if (this.ioHandler.failed) {
                var except = new LuaException(Objects.requireNonNull(this.ioHandler.from_wasm_poll()).asString());
                this.ioHandler.clear_all();
                throw except;
            }
            var rtn = this.ioHandler.getFrom_wasm().stream().map(IOValue::asObject).toArray();
            this.ioHandler.clear_all();
            return MethodResult.of(rtn);
        } catch (Exception e) {
            this.ioHandler.clear_all();
            throw new LuaException(e.getMessage());
        }
    }


}
