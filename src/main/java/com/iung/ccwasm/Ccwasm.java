package com.iung.ccwasm;

//import dan200.computercraft.api.ComputerCraftAPI;

import com.iung.ccwasm.api.Api1;
import dan200.computercraft.api.peripheral.PeripheralLookup;
import net.fabricmc.api.ModInitializer;
import dan200.computercraft.api.ComputerCraftAPI;
import net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents;
import net.fabricmc.fabric.api.object.builder.v1.block.FabricBlockSettings;
import net.minecraft.block.Block;
import net.minecraft.block.Blocks;
import net.minecraft.item.BlockItem;
import net.minecraft.item.Item;
import net.minecraft.registry.Registries;
import net.minecraft.registry.Registry;
import net.minecraft.server.MinecraftServer;
import net.minecraft.util.Identifier;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class Ccwasm implements ModInitializer {
//    public static Identifier RECEIVER_BLOCK_ID = Identifier.of(Ccwasm.MOD_ID, "receiver_block");
//    public static final Block RECEIVER_BLOCK = registerBlock(RECEIVER_BLOCK_ID, new Block(FabricBlockSettings.copyOf(Blocks.REDSTONE_BLOCK)));

    public static final String MOD_ID = "ccwasm";

    // This logger is used to write text to the console and the log file.
    // It is considered best practice to use your mod id as the logger's name.
    // That way, it's clear which mod wrote info, warnings, and errors.
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);
    public static final Path WASM_ROOT = Path.of("./wasm");
    public static volatile MinecraftServer SERVER = null;
    public static volatile Path SAVE_WASM_ROOT = null;

    @Override
    public void onInitialize() {
        ComputerCraftAPI.registerAPIFactory(Api1::new);
        ServerLifecycleEvents.SERVER_STARTED.register(server -> {
            Ccwasm.SERVER = server;
            Path saveWasm = server.getSavePath(net.minecraft.util.WorldSavePath.ROOT).resolve("computercraft").resolve("wasm");
            try {
                Files.createDirectories(saveWasm);
                Ccwasm.SAVE_WASM_ROOT = saveWasm;
            } catch (IOException e) {
                LOGGER.error("failed to create save wasm dir", e);
                Ccwasm.SAVE_WASM_ROOT = null;
            }
        });
        ServerLifecycleEvents.SERVER_STOPPED.register(server -> {
            Ccwasm.SERVER = null;
            Ccwasm.SAVE_WASM_ROOT = null;
        });
    }

//    private static Block registerBlock(Identifier id, Block block) {
//        registerBlockItem(id, block);
//        return Registry.register(Registries.BLOCK, id, block);
//    }
//
//    private static void registerBlockItem(Identifier id, Block block) {
//        Registry.register(Registries.ITEM, id, new BlockItem(block, new Item.Settings()));
//    }
}