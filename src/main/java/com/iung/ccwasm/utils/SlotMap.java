package com.iung.ccwasm.utils;

import java.util.HashMap;
import java.util.Map;

public class SlotMap<T> {
    private final Map<Integer, T> map;
    private int nextKey;

    public SlotMap() {
        map = new HashMap<>();
        nextKey = 1;
    }

    public T get(int idx) {
        return this.map.get(idx);
    }

    public void drop(int idx) {
        this.map.remove(idx);
    }

    public int count() {
        return map.size();
    }

    public int put(T data) {
        int key = nextKey++;
        if (nextKey <= 0) {
            nextKey = 1;
        }
        map.put(key, data);
        return key;
    }
}
