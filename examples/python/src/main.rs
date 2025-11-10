use std::{hint::black_box, time::Instant};

#[no_mangle]
pub extern "C" fn call() -> i32 {
    (b(1000000) * 1000.) as i32
}
fn b(c: usize) -> f64 {
    let ts = Instant::now();
    let mut num = black_box((black_box(32)));
    for _ in 0..c {
        num += black_box(1);
    }
    return ts.elapsed().as_secs_f64() * 1000_000_000. / c as f64;
}

fn main() {
    dbg!(call());
    // dbg!(b(10000000));
    // println!("{:?}", 'a');
}
