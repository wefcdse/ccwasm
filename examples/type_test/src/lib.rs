use cc_wasm_api::lua_api::LuaError;
use cc_wasm_api::prelude::*;
use std::cell::RefCell;

export_funcs!(
    version,
    position,
    check_bin,
    init,
    pi32,
    pi64,
    i64out,
    pf64,
    f32out,
    f64out,
    pstr,
    pbool,
    pnil,
    pnum,
    ptuple,
    f_multi,
    pvec,
    peither,
    pobj,
    obj_hold,
    obj_give,
    f_err,
    eval_test,
    eval_get
);

fn init() {}

fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// 编译期由 feature 决定：普通构建 = "global"，--features from_save = "save"
fn position() -> String {
    #[cfg(feature = "from_save")]
    {
        "save".to_owned()
    }
    #[cfg(not(feature = "from_save"))]
    {
        "global".to_owned()
    }
}

/// 二进制传输测试：与编译时嵌入的 bin_test.txt 逐字节比对
fn check_bin(data: Vec<u8>) -> String {
    let expected: &[u8] = include_bytes!("bin_test.txt");
    if data == expected {
        "match".to_owned()
    } else {
        let diff_count = data
            .iter()
            .zip(expected.iter())
            .filter(|(a, b)| a != b)
            .count();
        let first_diff = data
            .iter()
            .zip(expected.iter())
            .position(|(a, b)| a != b)
            .unwrap_or(usize::MAX);
        format!(
            "mismatch: len {} vs {}, diff {} bytes, first at {}",
            data.len(),
            expected.len(),
            diff_count,
            first_diff
        )
    }
}

fn pi32(x: i32) -> i32 {
    x
}

fn pi64(x: i64) -> i64 {
    x
}

fn i64out() -> i64 {
    1 << 40
}

fn pf64(x: f64) -> f64 {
    x
}

fn f32out() -> f32 {
    1.5
}

fn f64out() -> f64 {
    2.25
}

fn pstr(s: String) -> String {
    s
}

fn pbool(b: bool) -> bool {
    b
}

fn pnil(n: Option<i32>) -> Option<i32> {
    n
}

fn pnum(n: Number) -> Number {
    n
}

fn ptuple(a: i32, b: String, c: bool) -> (i32, String, bool) {
    (a, b, c)
}

fn f_multi() -> (i32, String) {
    (42, "multi".to_owned())
}

fn pvec(v: Vec<i32>) -> i32 {
    v.into_iter().sum()
}

fn peither(e: Either<String, Number>) -> String {
    match e {
        Either::First(s) => format!("str:{}", s),
        Either::Second(n) => format!("num:{}", n),
    }
}

fn pobj(o: LuaObject) -> LuaObject {
    o
}

static HELD: SyncNonSync<RefCell<Option<LuaObject>>> = SyncNonSync(RefCell::new(None));

fn obj_hold(o: LuaObject) {
    if let Some(old) = HELD.borrow_mut().replace(o) {
        old.drop();
    }
}

fn obj_give() -> Option<LuaObject> {
    HELD.borrow_mut().take()
}

fn f_err() -> LuaResult<i32> {
    Err(LuaError::from_str("test error"))
}

static EVAL: SyncNonSync<RefCell<Option<Number>>> = SyncNonSync(RefCell::new(None));

fn eval_test() {
    spawn(async {
        let n: Number = eval("return 1+2").await.unwrap();
        *EVAL.borrow_mut() = Some(n);
    });
}

fn eval_get() -> Option<Number> {
    EVAL.borrow_mut().take()
}
