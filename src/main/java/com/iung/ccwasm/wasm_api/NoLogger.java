package com.iung.ccwasm.wasm_api;

import com.dylibso.chicory.log.Logger;

public class NoLogger implements Logger {
    @Override
    public void log(Level level, String msg, Throwable throwable) {

    }

    @Override
    public boolean isLoggable(Level level) {
        return false;
    }
}
