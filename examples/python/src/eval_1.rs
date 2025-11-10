use std::cell::RefCell;
use std::future::Future;
use std::marker::{PhantomData, PhantomPinned};
use std::panic::{catch_unwind, PanicInfo};
use std::task::Poll;
use std::time::Duration;

use cc_wasm_api::coroutine::{
    coroutines, sleep, spawn, yield_counter, yield_now, CoroutineSpawn, UnsyncChannel,
};
use cc_wasm_api::eval::{eval, yield_lua};
use cc_wasm_api::export_funcs;
use cc_wasm_api::lua_api::{Exportable, Importable, LuaResult};
use cc_wasm_api::utils::{Debuged, Number, SyncNonSync};

export_funcs!((aa, init), (b, bb), (c, cc));
static C: SyncNonSync<UnsyncChannel<String>> = SyncNonSync(UnsyncChannel::new(10));

fn b(a: Option<Number>, b: Option<Number>) -> String {
    format!("{:?},{:?}", a, b)
}

fn c(a: Option<bool>) -> Option<bool> {
    a
}
// static INITED: SyncNonSync<RefCell<bool>> = SyncNonSync(RefCell::new(false));
fn aa() {
    let a = async {
        let mut cnt = 0;
        loop {
            // let l: LuaResult<Option<(Number, Number, Number)>> = eval("return gps.locate()").await;
            let () = eval(&format!("print(\"{}\")", cnt)).await.unwrap();
            cnt += 1;
            yield_now().await;
        }
    }
    .spawn();

    async {
        loop {
            let a: Number = eval("return redstone.getAnalogInput(\"left\")")
                .await
                .unwrap();
            let b = if a.to_i32() <= 7 { 15 } else { 0 };
            let () = eval(&format!("redstone.setAnalogOutput(\"right\",{})", b))
                .await
                .unwrap();

            yield_now().await;
        }
    }
    .spawn();

    async {
        loop {
            // sleep(Duration::from_secs(1)).await;
            if yield_counter() > coroutines() * 10 {}
            yield_lua().await;
        }
    }
    .spawn();

    async {
        yield_now().await;
        spawn(async {});
    }
    .spawn();

    async move {
        sleep(Duration::from_secs(1)).await;
        a.stop();
    }
    .spawn();

    // spawn(async {
    //     let a: String = async_eval("return \"32e1e212\"").await.unwrap();
    //     C.insert(a).await;
    // });
}
