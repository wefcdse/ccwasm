package com.iung.ccwasm.wasm_api;

import java.util.LinkedList;
import java.util.Queue;

public class IOHandler {

    public Queue<IOValue> to_wasm;
    public Queue<IOValue> from_wasm;
    public boolean failed;

    public IOHandler() {
        this.to_wasm = new LinkedList<>();
        this.from_wasm = new LinkedList<>();
        this.failed = false;
    }

    public void clear_all() {
        this.to_wasm.clear();
        this.from_wasm.clear();
        this.failed = false;
    }

    public void fail() {
        this.failed = true;
    }

    public void success() {
        this.failed = false;
    }

    public void push(IOValue v) {
        to_wasm.add(v);
    }

    public void push(int type, Object data) {
        to_wasm.add(new IOValue(type, data));
    }

    public IOValue pop() {
        return from_wasm.poll();
    }
}
