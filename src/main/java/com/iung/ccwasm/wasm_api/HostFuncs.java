package com.iung.ccwasm.wasm_api;

import com.dylibso.chicory.runtime.HostFunction;
import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.runtime.WasmFunctionHandle;
import com.dylibso.chicory.wasi.WasiOptions;
import com.dylibso.chicory.wasi.WasiPreview1;
import com.dylibso.chicory.wasm.types.Value;
import com.dylibso.chicory.wasm.types.ValueType;
import com.iung.ccwasm.Ccwasm;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;

public class HostFuncs {

    static final String MOD_NAME = "host";
    IOHandler ioHandler;

    static HostFunction build_host_fn(
            ValuedHostFuncHandler handle,
            String moduleName,
            String symbolName,
            List<ValueType> paramTypes,
            List<ValueType> returnTypes
    ) {
//        Value v = new Value()
        WasmFunctionHandle handle1 = (Instance instance, long... args) -> {
            Value[] a = new Value[args.length];
            for (int i = 0; i < args.length; i++) {
                a[i] = new Value(paramTypes.get(i), args[i]);
            }
            Value[] out = handle.apply(instance, a);
            if (out == null) {
                return null;
            }
            long[] out_long = new long[out.length];
            for (int i = 0; i < out.length; i++) {
                out_long[i] = out[i].raw();
            }
//            handle.apply(instance,);
            return out_long;
        };
        return new HostFunction(moduleName, symbolName, paramTypes, returnTypes, handle1);
    }

    public HostFuncs(IOHandler ioHandler) {
        this.ioHandler = ioHandler;
    }

    public HostFunction[] wasi() {
        var fakeStdin = new ByteArrayInputStream("".getBytes());
// We will create two output streams to capture stdout and stderr
        var fakeStdout = new ByteArrayOutputStream();
        var fakeStderr = new ByteArrayOutputStream();
// now pass those to our wasi options builder
        var wasiOpts = WasiOptions.builder().withStdout(fakeStdout).withStderr(fakeStderr).withStdin(fakeStdin).build();
        var wasi = WasiPreview1
                .builder()
                .withLogger(new NoLogger())
                .withOptions(wasiOpts)
                .build();
        return wasi.toHostFunctions();
    }

    public HostFunction[] all() {
        return new HostFunction[]{
                next_type(),
                import_string_length(),
                import_string_data(),
                export_string(),
                import_i32(),
                import_i64(),
                import_f32(),
                import_f64(),
                export_i32(),
                export_i64(),
                export_f32(),
                export_f64(),
                abort_next_import(),
                success(),
                failed(),
                import_obj(),
                export_obj(),
                drop_obj(),
                call_eval(), eval_ready(),
                clear_eval(), import_from_eval(),
                import_bool(), export_bool(),
                export_nil()
        };
    }

    public static HostFunction show_str() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            var base_addr = args[0].asInt();
            var len = args[1].asInt();
            var msg = instance.memory().readString(base_addr, len);
            System.out.println(msg);
            return null;
        }, MOD_NAME, "show_str", List.of(ValueType.I32, ValueType.I32), List.of());
    }

    public HostFunction next_type() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            var first = ioHandler.to_wasm_peek();
            if (first == null) {
                return new Value[]{Value.i32(0)};
            }
            return new Value[]{Value.i32(first.type)};
        }, MOD_NAME, "next_type", List.of(), List.of(ValueType.I32));
    }

    public HostFunction import_string_length() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            var first = ioHandler.to_wasm_peek();
            var len = ((byte[]) Objects.requireNonNull(first).data).length;
            return new Value[]{Value.i32(len)};
        }, MOD_NAME, "import_string_length", List.of(), List.of(ValueType.I32));
    }

    public HostFunction import_string_data() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            var addr = args[0].asInt();
            var first = ioHandler.to_wasm_poll();
            var mem = instance.memory();
            byte[] string = (byte[]) Objects.requireNonNull(first).data;
            mem.write(addr, string);
            return null;
        }, MOD_NAME, "import_string_data", List.of(ValueType.I32), List.of());
    }

    public HostFunction export_string() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            var base_addr = args[0].asInt();
            var len = args[1].asInt();
            var str = instance.memory().readString(base_addr, len);
            ioHandler.from_wasm_push(IOValue.of(str));
            return null;
        }, MOD_NAME, "export_string", List.of(ValueType.I32, ValueType.I32), List.of());
    }

    public HostFunction import_i32() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            int data = Objects.requireNonNull(ioHandler.to_wasm_poll()).asInt();
            return new Value[]{Value.i32(data)};
        }, MOD_NAME, "import_i32", List.of(), List.of(ValueType.I32));
    }

    public HostFunction import_i64() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            long data = Objects.requireNonNull(ioHandler.to_wasm_poll()).asLong();
            return new Value[]{Value.i64(data)};
        }, MOD_NAME, "import_i64", List.of(), List.of(ValueType.I64));
    }

    public HostFunction import_f32() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            float data = Objects.requireNonNull(ioHandler.to_wasm_poll()).asFloat();
            return new Value[]{Value.fromFloat(data)};
        }, MOD_NAME, "import_f32", List.of(), List.of(ValueType.F32));
    }

    public HostFunction import_f64() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            double data = Objects.requireNonNull(ioHandler.to_wasm_poll()).asDouble();
            return new Value[]{Value.fromDouble(data)};
        }, MOD_NAME, "import_f64", List.of(), List.of(ValueType.F64));
    }

    public HostFunction export_i32() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            int value = args[0].asInt();
            ioHandler.from_wasm_push(IOValue.of(value));
            return null;
        }, MOD_NAME, "export_i32", List.of(ValueType.I32), List.of());
    }

    public HostFunction export_i64() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            long value = args[0].asLong();
            ioHandler.from_wasm_push(IOValue.of(value));
            return null;
        }, MOD_NAME, "export_i64", List.of(ValueType.I64), List.of());
    }

    public HostFunction export_f32() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            float value = args[0].asFloat();
            ioHandler.from_wasm_push(IOValue.of(value));
            return null;
        }, MOD_NAME, "export_f32", List.of(ValueType.F32), List.of());
    }

    public HostFunction export_f64() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            double value = args[0].asDouble();
            ioHandler.from_wasm_push(IOValue.of(value));
            return null;
        }, MOD_NAME, "export_f64", List.of(ValueType.F64), List.of());
    }

    public HostFunction abort_next_import() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            ioHandler.to_wasm_poll();
            return null;
        }, MOD_NAME, "abort_next_import", List.of(), List.of());
    }

    public HostFunction success() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            ioHandler.success();
            return null;
        }, MOD_NAME, "success", List.of(), List.of());
    }

    public HostFunction failed() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            ioHandler.fail();
            return null;
        }, MOD_NAME, "failed", List.of(), List.of());
    }

    public HostFunction import_obj() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            int key = 0;
            try {
                IOValue obj = ioHandler.to_wasm_poll();
//                Ccwasm.LOGGER.info("{}", obj);

                key = ioHandler.obj_hold.put(Objects.requireNonNull(obj).data);
            } catch (Exception e) {
                Ccwasm.LOGGER.info("{}", e.getMessage());
            }
            return new Value[]{Value.i32(key)};
        }, MOD_NAME, "import_obj", List.of(), List.of(ValueType.I32));
    }

    public HostFunction export_obj() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            int key = args[0].asInt();
            var obj = ioHandler.obj_hold.get(key);
            ioHandler.from_wasm_push(IOValue.of_obj(obj));
            return null;
        }, MOD_NAME, "export_obj", List.of(ValueType.I32), List.of());
    }

    public HostFunction drop_obj() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            int key = args[0].asInt();
            ioHandler.obj_hold.drop(key);
            return null;
        }, MOD_NAME, "drop_obj", List.of(ValueType.I32), List.of());
    }

    public HostFunction call_eval() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            if (!this.ioHandler.eval_valid()) {
                return new Value[]{Value.i32(0)};
            }
            var base_addr = args[0].asInt();
            var len = args[1].asInt();
            var str = instance.memory().readString(base_addr, len);
            ioHandler.setTo_eval(str);
            return new Value[]{Value.i32(1)};
        }, MOD_NAME, "call_eval", List.of(ValueType.I32, ValueType.I32), List.of(ValueType.I32));
    }

    public HostFunction eval_ready() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            return new Value[]{Value.i32(this.ioHandler.eval_ready() ? 1 : 0)};
        }, MOD_NAME, "eval_ready", List.of(), List.of(ValueType.I32));
    }

    public HostFunction clear_eval() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            ioHandler.clear_eval();
            return null;
        }, MOD_NAME, "clear_eval", List.of(), List.of());
    }

    public HostFunction import_from_eval() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            ioHandler.import_from_eval();
            return null;
        }, MOD_NAME, "import_from_eval", List.of(), List.of());
    }

    public HostFunction import_bool() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            boolean data = (boolean) Objects.requireNonNull(ioHandler.to_wasm_poll()).data;
            return new Value[]{Value.i32(data ? 1 : 0)};
        }, MOD_NAME, "import_bool", List.of(), List.of(ValueType.I32));
    }

    public HostFunction export_bool() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            int value = args[0].asInt();
            ioHandler.from_wasm_push(new IOValue(IOValue.Bool, value != 0));
            return null;
        }, MOD_NAME, "export_bool", List.of(ValueType.I32), List.of());
    }

    public HostFunction export_nil() {
        return build_host_fn((Instance instance, Value... args) -> { // decompiled is: console_log(13, 0);
            ioHandler.from_wasm_push(new IOValue(IOValue.Nil, null));
            return null;
        }, MOD_NAME, "export_nil", List.of(), List.of());
    }
}
