package com.iung.ccwasm.utils;

import java.util.HashMap;
import java.util.Map;
import java.util.Random;

public class SlotMap<T> {
    private final Map<Integer, T> map;
    private Random rand;

    public SlotMap() {
        map = new HashMap<>();
        rand = new Random();
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
        while (true) {
            int key = rand.nextInt();
            if (!map.containsKey(key) && key != 0) {
                map.put(key, data);
                return key;
            }
        }
    }
}
