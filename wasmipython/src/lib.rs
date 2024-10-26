use std::borrow::BorrowMut;

use wasmi::{errors::LinkerError, AsContextMut, Caller, Func, Linker, Store};

mod wasi_wasi {
    mod ffi {
        #[link(wasm_import_module = "wasi_snapshot_preview1")]
        extern "C" {
            pub fn clock_time_get(a: i32, b: i64, c: i32) -> i32;
            pub fn fd_write(a: i32, b: i32, c: i32, d: i32) -> i32;
            pub fn environ_get(a: i32, b: i32) -> i32;
            pub fn environ_sizes_get(a: i32, b: i32) -> i32;
            pub fn proc_exit(a: i32);
        }
    }
    pub fn clock_time_get(a: i32, b: i64, c: i32) -> i32 {
        unsafe { ffi::clock_time_get(a, b, c) }
    }
    pub fn fd_write(a: i32, b: i32, c: i32, d: i32) -> i32 {
        unsafe { ffi::fd_write(a, b, c, d) }
    }
    pub fn environ_get(a: i32, b: i32) -> i32 {
        unsafe { ffi::environ_get(a, b) }
    }
    pub fn environ_sizes_get(a: i32, b: i32) -> i32 {
        unsafe { ffi::environ_sizes_get(a, b) }
    }
    pub fn proc_exit(a: i32) {
        unsafe { ffi::proc_exit(a) }
    }
}

pub fn link<T>(store: &mut Store<T>, linker: &mut Linker<T>) -> Result<(), LinkerError> {
    use wasi_wasi::*;

    {
        let clock_time_get = Func::wrap(
            store.borrow_mut(),
            |mut caller: Caller<'_, T>, a: i32, b: i64, c: i32| -> i32 {
                let m = caller.get_export("memory").unwrap().into_memory().unwrap();
                let ptr_base = m.data_ptr(caller.as_context_mut()) as usize as i32;
                clock_time_get(a, b, ptr_base + c)
            },
        );
        linker.define("wasi_snapshot_preview1", "clock_time_get", clock_time_get)?;
    }

    {
        let clock_time_get = Func::wrap(
            store.borrow_mut(),
            |mut caller: Caller<'_, T>, a: i32, b: i32, c: i32, d: i32| -> i32 {
                // dbg!(a, b, c, d);
                let m = caller.get_export("memory").unwrap().into_memory().unwrap();
                let ptr_base = m.data_ptr(caller.as_context_mut()) as usize as i32;
                fd_write(a, b + ptr_base, c, d + ptr_base)
            },
        );
        linker.define("wasi_snapshot_preview1", "fd_write", clock_time_get)?;
    }

    linker.func_wrap("wasi_snapshot_preview1", "environ_get", environ_get)?;
    linker.func_wrap(
        "wasi_snapshot_preview1",
        "environ_sizes_get",
        environ_sizes_get,
    )?;
    linker.func_wrap("wasi_snapshot_preview1", "proc_exit", proc_exit)?;

    Ok(())
}
