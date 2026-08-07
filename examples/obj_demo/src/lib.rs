use cc_wasm_api::{debug::show_str, prelude::*};
use std::cell::RefCell;

export_funcs!(init, pass, hold, give, drop_held);

static HELD: SyncNonSync<RefCell<Option<LuaObject>>> = SyncNonSync(RefCell::new(None));

fn init() {
    show_str("obj_demo init");
}

/// 原样把 lua table 传回 lua，验证 slot map 的 key 分配正确
fn pass(t: LuaObject) -> LuaObject {
    show_str("pass: table passed through");
    t
}

/// 保存一个 table，稍后 give() 取回
fn hold(t: LuaObject) {
    if let Some(old) = HELD.borrow_mut().replace(t) {
        old.drop();
        show_str("hold: previous object released");
    }
    show_str("hold: table saved");
}

/// 返回之前 hold 的 table，未被 hold 过则返回空
fn give() -> Option<LuaObject> {
    match HELD.borrow_mut().take() {
        Some(t) => {
            show_str("give: table returned");
            Some(t)
        }
        None => {
            show_str("give: nothing held");
            None
        }
    }
}

/// 保存后立刻释放 slot，之后 give 拿不到
fn drop_held() {
    if let Some(t) = HELD.borrow_mut().take() {
        t.drop();
        show_str("drop_held: slot released");
    }
}
