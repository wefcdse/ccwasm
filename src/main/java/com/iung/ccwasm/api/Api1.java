package com.iung.ccwasm.api;


//import com.dylibso.chicory.runtime.Module;

import com.iung.ccwasm.Ccwasm;
import com.iung.ccwasm.WasmCtx;
import dan200.computercraft.api.ComputerCraftAPI;
import dan200.computercraft.api.filesystem.WritableMount;
import dan200.computercraft.api.lua.*;
import dan200.computercraft.core.filesystem.FileMount;
import net.minecraft.server.MinecraftServer;


import java.io.File;
import java.nio.file.Path;
//import dan200.computercraft.api.detail.;


public class Api1 implements ILuaAPI {
    static String[] TARGET_GLO = {"wasm"};
    static FileMount WASM_ROOT;
    public static final String SHARED_NAME = "shared";

    static {
        WASM_ROOT = new FileMount(Ccwasm.WASM_ROOT);
    }

    private final IComputerSystem cs;

    public Api1(IComputerSystem iComputerSystem) {
        cs = iComputerSystem;
    }


    @Override
    public String[] getNames() {
        return TARGET_GLO;
    }


    @LuaFunction
    public final WasmCtx load_wasm(ILuaContext ctx, IArguments args) throws LuaException {
        try {
            Path p = Path.of(args.getString(0).chars().filter(c -> Character.isDigit(c) | Character.isAlphabetic(c) | c == '_' | c == '-').collect(StringBuilder::new, StringBuilder::appendCodePoint,
                    StringBuilder::append) + ".wasm");
            Path p1 = Ccwasm.WASM_ROOT.resolve(p);
            boolean useAoT = true;
            if (args.count() >= 2 && args.get(1) instanceof Boolean) {
                useAoT = (Boolean) args.get(1);
//                Ccwasm.LOGGER.info("use aot: {}", useAoT);
//                Ccwasm.LOGGER.info("type: {}", args.getType(1));
            }
            return new WasmCtx(p1.toFile(), useAoT);
        } catch (Exception e) {
            throw new LuaException(e.getMessage());
        }
    }

//    @LuaFunction
//    public final void type(ILuaContext ctx, IArguments args) throws LuaException {
//        Ccwasm.LOGGER.info("{}", args.getType(0));
//        Ccwasm.LOGGER.info("{}", ctx.getClass());
//        Ccwasm.LOGGER.info("{}", args.getClass());
//    }

    @Override
    public void startup() {
        cs.mount("wasm", WASM_ROOT);
        MinecraftServer server = Ccwasm.SERVER;
        if (server != null) {
            WritableMount shared = ComputerCraftAPI.createSaveDirMount(server, SHARED_NAME, 1024L * 1024 * 1024);
            cs.mountWritable(SHARED_NAME, shared);
        } else {
            Ccwasm.LOGGER.warn("server not started, skip mounting shared");
        }
        ILuaAPI.super.startup();
    }

    @Override
    public void shutdown() {
        cs.unmount("wasm");
        cs.unmount(SHARED_NAME);
        ILuaAPI.super.shutdown();
    }
}
