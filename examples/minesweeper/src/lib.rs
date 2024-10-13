use cc_wasm_api::{
    addon::{
        local_monitor::LocalMonitor,
        misc::{ColorId, Side},
    },
    prelude::*,
};
use functions::poll_evt;
use ms::{game_logic, GameState};

mod functions;
mod ms;

export_funcs!(init);
static CLICKED: SyncNonSync<UnsyncChannel<(i32, i32)>> = SyncNonSync(UnsyncChannel::new(0));
static MONITOR: SyncNonSync<AsyncLock<LocalMonitor>> =
    SyncNonSync(AsyncLock::new(LocalMonitor::new_empty(SIDE)));
// static B: SyncNonSync<Syncer> = SyncNonSync(Syncer::new());
const SIDE: Side = Side::Top;

fn init() {
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
        MONITOR.lock().await.init().await.unwrap();
        MONITOR.lock().await.clear(ColorId::White).await.unwrap();
        loop {
            MONITOR.lock().await.sync().await.unwrap();
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
        game_logic(&mut gs, &mut *MONITOR.lock().await).await;
        ts.sync().await;
    }
}
