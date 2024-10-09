use cc_wasm_api::prelude::*;
use functions::{init_monitor, poll_evt};
use local_monitor::LocalMonitor;
use ms::{game_logic, GameState};
use simple_cell::{SimpleCell, Syncer};
use utils::{AsIfPixel, ColorId};

mod functions;
mod local_monitor;
mod ms;
mod simple_cell;
mod utils;
mod vec2d;

export_funcs!(init);
static CLICKED: SyncNonSync<UnsyncChannel<(i32, i32)>> = SyncNonSync(UnsyncChannel::new(0));
static MONITOR: SyncNonSync<SimpleCell<LocalMonitor>> = SyncNonSync(SimpleCell::new());
static B: SyncNonSync<Syncer> = SyncNonSync(Syncer::new());

fn init() {
    MONITOR.init(LocalMonitor::new(
        0,
        0,
        AsIfPixel::new(' ', ColorId::Blue, ColorId::Orange).unwrap(),
    ));
    async {
        loop {
            poll_evt().await;
            unsafe { TickSyncer::handle_sync() }.await;
        }
    }
    .spawn();

    // main logic

    main_logic().spawn();

    async {
        let mut ts = TickSyncer::new();
        init_monitor().await;
        let size: (Number, Number) = eval("return monitor.getSize()").await.unwrap();
        MONITOR.get().resize(
            size.0.to_i32() as usize,
            size.1.to_i32() as usize,
            AsIfPixel::new(' ', ColorId::Blue, ColorId::Orange).unwrap(),
        );
        MONITOR.get().sync_all().await;
        loop {
            B.wait(1).await;
            MONITOR.get().sync().await;
            ts.sync().await;
        }
    }
    .spawn();
    async {
        let mut ts = TickSyncer::new();
        loop {
            // for i in 0..10 {
            //     let p = AsIfPixel::new(
            //         ' ',
            //         ColorId::from_number_overflow(random()),
            //         ColorId::from_number_overflow(random()),
            //     )
            //     .unwrap();
            //     MONITOR.get().write(1, MONITOR.get().y() - i, p);
            // }
            B.notify();
            ts.sync().await;
        }
    }
    .spawn();
}

async fn main_logic() {
    let mut ts = TickSyncer::new();
    let mut gs = GameState::StartUp;
    loop {
        // while let Some((x, y)) = CLICKED.try_get() {

        //     ////// proc click
        //     let p = AsIfPixel::new(' ', ColorId::Black, ColorId::Black).unwrap();
        //     MONITOR.get().write(x as usize, y as usize, p);
        // }
        game_logic(&mut gs, MONITOR.get()).await;
        B.notify();
        ts.sync().await;
    }
}
