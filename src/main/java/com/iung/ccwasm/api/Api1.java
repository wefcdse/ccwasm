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
            File file = resolveWasmFile(args);
            return new WasmCtx(file, useAoT(args));
        } catch (Exception e) {
            throw new LuaException(e.getMessage());
        }
    }

    @LuaFunction
    public final void precompile(ILuaContext ctx, IArguments args) throws LuaException {
        try {
            File file = resolveWasmFile(args);
            WasmCtx.precompile(wasmName(args), file);
        } catch (Exception e) {
            throw new LuaException(e.getMessage());
        }
    }

    @LuaFunction
    public final String precompile_done(ILuaContext ctx, IArguments args) throws LuaException {
        try {
            return WasmCtx.precompileStatus(wasmName(args));
        } catch (Exception e) {
            throw new LuaException(e.getMessage());
        }
    }

    static String wasmName(IArguments args) throws LuaException {
        return args.getString(0).chars().filter(c -> Character.isDigit(c) | Character.isAlphabetic(c) | c == '_' | c == '-').collect(StringBuilder::new, StringBuilder::appendCodePoint,
                StringBuilder::append).toString();
    }

    static boolean useAoT(IArguments args) throws LuaException {
        if (args.count() >= 2 && args.get(1) instanceof Boolean) {
            return (Boolean) args.get(1);
        }
        return true;
    }

    static File resolveWasmFile(IArguments args) throws LuaException {
        Path p = Path.of(wasmName(args) + ".wasm");
        String source = "auto";
        if (args.count() >= 3 && args.get(2) instanceof String) {
            source = (String) args.get(2);
        }
        Path globalFile = Ccwasm.WASM_ROOT.resolve(p);
        Path saveFile = Ccwasm.SAVE_WASM_ROOT == null ? null : Ccwasm.SAVE_WASM_ROOT.resolve(p);
        switch (source) {
            case "global" -> {
                return globalFile.toFile();
            }
            case "save" -> {
                if (saveFile != null && saveFile.toFile().exists()) {
                    return saveFile.toFile();
                }
                throw new LuaException("wasm not found in save dir: " + p);
            }
            default -> {
                return (saveFile != null && saveFile.toFile().exists())
                        ? saveFile.toFile()
                        : globalFile.toFile();
            }
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
        if (Ccwasm.SAVE_WASM_ROOT != null) {
            cs.mount("wasm_save", new FileMount(Ccwasm.SAVE_WASM_ROOT));
        }
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
        cs.unmount("wasm_save");
        cs.unmount(SHARED_NAME);
        ILuaAPI.super.shutdown();
    }
}
