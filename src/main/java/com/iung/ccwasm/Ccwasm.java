package com.iung.ccwasm;

import com.iung.ccwasm.api.Api1;
import dan200.computercraft.api.ComputerCraftAPI;
import net.minecraft.server.MinecraftServer;
import net.minecraft.world.level.storage.LevelResource;
import net.neoforged.bus.api.IEventBus;
import net.neoforged.fml.common.Mod;
import net.neoforged.fml.loading.FMLPaths;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.server.ServerStartedEvent;
import net.neoforged.neoforge.event.server.ServerStoppedEvent;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

@Mod(Ccwasm.MOD_ID)
public class Ccwasm {
    public static final String MOD_ID = "ccwasm";

    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);
    public static final Path WASM_ROOT = FMLPaths.GAMEDIR.get().resolve("wasm");
    public static volatile MinecraftServer SERVER = null;
    public static volatile Path SAVE_WASM_ROOT = null;

    public Ccwasm(IEventBus modEventBus) {
        ComputerCraftAPI.registerAPIFactory(Api1::new);
        NeoForge.EVENT_BUS.addListener(Ccwasm::onServerStarted);
        NeoForge.EVENT_BUS.addListener(Ccwasm::onServerStopped);
    }

    private static void onServerStarted(ServerStartedEvent event) {
        MinecraftServer server = event.getServer();
        Ccwasm.SERVER = server;
        Path saveWasm = server.getWorldPath(LevelResource.ROOT).resolve("computercraft").resolve("wasm");
        try {
            Files.createDirectories(saveWasm);
            Ccwasm.SAVE_WASM_ROOT = saveWasm;
        } catch (IOException e) {
            LOGGER.error("failed to create save wasm dir", e);
            Ccwasm.SAVE_WASM_ROOT = null;
        }
    }

    private static void onServerStopped(ServerStoppedEvent event) {
        Ccwasm.SERVER = null;
        Ccwasm.SAVE_WASM_ROOT = null;
    }
}
