use cc_wasm_api::prelude::*;
use std::time::{Duration, Instant};

export_funcs!(init, aa);

fn aa(a: Either<Number, Nil>, b: String) -> (bool, Either<String, Nil>, i32) {
    (true, Either::First(format!("{:?}, {}", a, b)), 3)
}

fn init() {
    TickSyncer::spawn_handle_coroutine();
    async {
        let mut ts = TickSyncer::new();
        let () = eval(&format!("redstone.setAnalogOutput(\"right\",{})", 5))
            .await
            .unwrap();
        ts.sleep(Duration::from_secs(3)).await;
        let () = eval(&format!("redstone.setAnalogOutput(\"right\",{})", 15))
            .await
            .unwrap();
        ts.sleep(Duration::from_secs(3)).await;
        sleep(Duration::from_secs(5)).await;

        ts.sync().await;
    }
    .spawn();
    async {
        let mut ts = TickSyncer::new();

        let start = Instant::now();
        loop {
            // let l: LuaResult<Option<(Number, Number, Number)>> = eval("return gps.locate()").await;
            let () = eval(&format!("print(\"{}\")", start.elapsed().as_secs_f32()))
                .await
                .unwrap();

            yield_now().await;
            yield_now().await;
            ts.sync().await;
        }
    }
    .spawn();
    async {
        let mut ts = TickSyncer::new();

        let mut cnt = 0;
        loop {
            // let l: LuaResult<Option<(Number, Number, Number)>> = eval("return gps.locate()").await;
            let () = eval(&format!("print(\"{}\")", cnt)).await.unwrap();

            cnt += 1;
            yield_now().await;
            yield_now().await;
            ts.sync().await;
        }
    }
    .spawn();
}
