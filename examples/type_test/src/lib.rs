use cc_wasm_api::lua_api::LuaError;
use cc_wasm_api::prelude::*;
use std::cell::RefCell;

export_funcs!(
    version,
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
