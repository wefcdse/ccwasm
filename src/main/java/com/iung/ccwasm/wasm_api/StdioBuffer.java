package com.iung.ccwasm.wasm_api;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.util.LinkedList;
import java.util.Queue;

/**
 * 可编程的 WASI stdin/stdout/stderr。
 * stdin 以队列形式接收 Lua 推入的数据（空时返回 EOF，不会阻塞电脑线程）；
 * stdout/stderr 累积输出，Lua 读取时取出并清空。
 */
public class StdioBuffer {

    private final Queue<byte[]> stdinQueue = new LinkedList<>();
    private final ByteArrayOutputStream stdoutBuf = new ByteArrayOutputStream();
    private final ByteArrayOutputStream stderrBuf = new ByteArrayOutputStream();

    private final InputStream stdin = new InputStream() {
        private byte[] cur;
        private int pos;

        @Override
        public int read() {
            if (cur == null || pos >= cur.length) {
                cur = pollStdin();
                pos = 0;
                if (cur == null) {
                    return -1;
                }
            }
            return cur[pos++] & 0xFF;
        }
    };

    public synchronized void pushStdin(byte[] data) {
        stdinQueue.add(data);
    }

    private synchronized byte[] pollStdin() {
        return stdinQueue.poll();
    }

    public InputStream stdin() {
        return stdin;
    }

    public synchronized void writeStdout(byte[] data) {
        stdoutBuf.writeBytes(data);
    }

    public synchronized void writeStderr(byte[] data) {
        stderrBuf.writeBytes(data);
    }

    public synchronized byte[] readStdout() {
        byte[] out = stdoutBuf.toByteArray();
        stdoutBuf.reset();
        return out;
    }

    public synchronized byte[] readStderr() {
        byte[] out = stderrBuf.toByteArray();
        stderrBuf.reset();
        return out;
    }
}
