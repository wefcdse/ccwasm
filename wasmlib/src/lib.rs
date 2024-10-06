use std::future::Future;
use std::marker::{PhantomData, PhantomPinned};
use std::task::Poll;
use std::time::Duration;

use cc_wasm_api::coroutine::{sleep, spawn, yield_now, UnsyncChannel};
use cc_wasm_api::eval::{eval, yield_lua};
use cc_wasm_api::export_funcs;
use cc_wasm_api::lua_api::{Exportable, Importable, LuaResult};
use cc_wasm_api::utils::{Debuged, Number, SyncNonSync};

export_funcs!((aa, init), (b, bb));
static C: SyncNonSync<UnsyncChannel<String>> = SyncNonSync(UnsyncChannel::new(10));

fn b(a: Option<Number>, b: Option<Number>) -> String {
    format!("{:?},{:?}", a, b)
}

fn aa() {
    spawn(async {
        loop {
            let a: Number = eval("return redstone.getAnalogInput(\"left\")")
                .await
                .unwrap();
            let () = eval(&format!("redstone.setAnalogOutput(\"right\",{})", a))
                .await
                .unwrap();
            let l: LuaResult<Option<(Number, Number, Number)>> = eval("return gps.locate()").await;
            let () = eval(&format!("print(\"{:?}\")", l)).await.unwrap();
            yield_now().await;
        }
    });
    spawn(async {
        loop {
            sleep(Duration::from_secs(1)).await;
            yield_lua().await;
        }
    });

    // spawn(async {
    //     let a: String = async_eval("return \"32e1e212\"").await.unwrap();
    //     C.insert(a).await;
    // });
}
