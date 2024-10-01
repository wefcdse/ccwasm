package com.iung.ccwasm;

import com.dylibso.chicory.runtime.*;
import com.dylibso.chicory.runtime.Module;
import com.dylibso.chicory.wasm.types.Export;
import com.iung.ccwasm.wasm_api.HostFuncs;
import com.iung.ccwasm.wasm_api.IOHandler;
import com.iung.ccwasm.wasm_api.IOValue;
import dan200.computercraft.api.lua.*;

import java.io.File;
import java.util.Objects;
import java.util.Set;
import java.util.logging.Logger;

public class WasmCtx implements IDynamicLuaObject {
    static String[] METHODS = {"load_wasm", "run_func"};
    IOHandler ioHandler;
    Module wasm_module;
    Instance wasm_instance;
    String[] methods;

    public WasmCtx(File file) {

        IOHandler io = new IOHandler();
        HostFuncs hfs = new HostFuncs(io);

        HostImports hi = HostImports.builder()
                .addFunction(HostFuncs.show_str())
                .addFunction(hfs.all())
                .addFunction(hfs.wasi())
                .build();

        this.ioHandler = io;
        this.wasm_module = Module.builder(file).withHostImports(hi).build();
        this.wasm_instance = wasm_module.instantiate();

        ExportFunction a = this.wasm_instance.export("export_func");
        a.apply();
        this.methods = this.ioHandler.from_wasm.stream().map(IOValue::asString).toArray(String[]::new);
        this.ioHandler.clear_all();
    }

    @Override
    public String[] getMethodNames() {
        return methods;
    }

    @Override
    public MethodResult callMethod(ILuaContext context, int method, IArguments arguments) throws LuaException {
//        Ccwasm.LOGGER.info("wasm ctx: Slots {}", ioHandler.obj_hold.count());
        try {
            this.ioHandler.clear_all();
            for (int i = 0; i < arguments.count(); i++) {
                ioHandler.push(IOValue.of_obj(arguments.get(i)));
            }
            ExportFunction a = this.wasm_instance.export(methods[method]);
            a.apply();
            if (this.ioHandler.failed) {
                var except = new LuaException(Objects.requireNonNull(this.ioHandler.from_wasm.poll()).asString());
                this.ioHandler.clear_all();
                throw except;
            }
            var rtn = this.ioHandler.from_wasm.stream().map(IOValue::asObject).toArray();
            this.ioHandler.clear_all();
            return MethodResult.of(rtn);
        } catch (Exception e) {
            this.ioHandler.clear_all();
            throw new LuaException(e.getMessage());
        }
    }


}
