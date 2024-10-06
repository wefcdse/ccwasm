package com.iung.ccwasm.wasm_api;

import com.iung.ccwasm.utils.SlotMap;

import javax.annotation.Nullable;
import java.util.LinkedList;
import java.util.Queue;

public class IOHandler {

    private Queue<IOValue> to_wasm;

    public Queue<IOValue> getFrom_wasm() {
        return from_wasm;
    }

    private Queue<IOValue> from_wasm;
    public SlotMap obj_hold;
    public boolean failed;

    private Queue<IOValue> to_wasm_eval;
    private boolean to_wasm_is_eval;


    @Nullable
    public String getTo_eval() {
        return to_eval;
    }

    public void setTo_eval(@Nullable String to_eval) {
        this.to_eval = to_eval;
    }

    @Nullable
    private String to_eval;

    public IOHandler() {
        this.to_wasm = new LinkedList<>();
        this.from_wasm = new LinkedList<>();
        this.failed = false;
        this.obj_hold = new SlotMap();
        this.to_eval = null;
        this.to_wasm_is_eval = false;
        this.to_wasm_eval = new LinkedList<>();
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

    public void to_wasm_push(IOValue v) {
        to_wasm.add(v);
    }

    public void to_wasm_push(int type, Object data) {
        to_wasm.add(new IOValue(type, data));
    }

    public IOValue to_wasm_poll() {
        return to_wasm_is_eval ? to_wasm_eval.poll() : to_wasm.poll();
    }

    public IOValue to_wasm_peek() {
        return to_wasm_is_eval ? to_wasm_eval.peek() : to_wasm.peek();
    }

    public IOValue from_wasm_poll() {
        return from_wasm.poll();
    }

    public void from_wasm_push(IOValue v) {
        from_wasm.add(v);
    }

    public void from_wasm_push(int type, Object data) {
        from_wasm.add(new IOValue(type, data));
    }

    public boolean eval_valid() {
        return to_eval == null && to_wasm_eval.isEmpty();
    }

    public boolean eval_ready() {
        return to_eval == null;
    }

    public void clear_eval() {
        this.to_eval = null;
        this.to_wasm_eval.clear();
        this.to_wasm_is_eval = false;
    }

    public void import_from_eval() {
        this.to_wasm_is_eval = true;
    }
    public void to_eval_push(IOValue v) {
        to_wasm_eval.add(v);
    }

}
