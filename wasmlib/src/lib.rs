use std::{
    hint::black_box,
    mem::transmute,
    ops::AddAssign,
    time::{Duration, Instant},
};

// export_funcs!(call);
#[no_mangle]
pub extern "C" fn call() -> f64 {
    dbg!(2);
    b(1_000_000_0)
}
fn b(c: usize) -> f64 {
    let ts = Instant::now();
    let mut num = black_box((black_box(32)));
    for _ in 0..c {
        num.add_assign(black_box(1));
    }
    black_box(num);
    let d = unsafe { transmute::<Instant, Duration>(ts) };
    return ts.elapsed().as_secs_f64() * 1000_000_000. / c as f64;
    return d.as_secs_f64();
}
