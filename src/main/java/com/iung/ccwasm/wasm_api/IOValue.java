package com.iung.ccwasm.wasm_api;

import dan200.computercraft.api.lua.LuaValues;

public class IOValue {
    public static final int I32 = 1;
    public static final int I64 = 2;
    public static final int String = 3;
    public static final int F32 = 4;
    public static final int F64 = 5;
    public static final int Type = 6;
    public static final int Table = 7;
    public static final int Nil = 8;

    public int type;
    public Object data;

    public IOValue(int type, Object data) {
        this.data = data;
        this.type = type;
    }


    public String asString() {
        return (java.lang.String) data;
    }

    public int asInt() {
        return (int) data;
    }

    public long asLong() {
        return (long) data;
    }

    public float asFloat() {
        return (float) data;
    }

    public double asDouble() {
        return (double) data;
    }

    public Object asObject() {
        return data;
    }

    public static IOValue of(int data) {
        return new IOValue(I32, data);
    }

    public static IOValue of(long data) {
        return new IOValue(I64, data);
    }

    public static IOValue of(String data) {
        return new IOValue(String, data);
    }

    public static IOValue of(float data) {
        return new IOValue(F32, data);
    }

    public static IOValue of(double data) {
        return new IOValue(F64, data);
    }

    public static IOValue type(int type) {
        return new IOValue(Type, type);
    }

    public static IOValue of_obj(Object data) {
        if (data instanceof Integer) {
            return IOValue.of((int) data);
        }
        if (data instanceof Long) {
            return IOValue.of((long) data);
        }
        if (data instanceof Float) {
            return IOValue.of((float) data);
        }
        if (data instanceof Double) {
            return IOValue.of((double) data);
        }
        if (data instanceof String) {
            return IOValue.of((String) data);
        }
        return new IOValue(Table, data);
    }

}
