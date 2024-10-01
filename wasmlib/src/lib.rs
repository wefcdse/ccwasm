use std::cell::LazyCell;
use std::fmt::Debug;
use std::sync::Mutex;

use lua_api::{Exportable, Importable, LuaResult};
use rustpython_vm::compiler::Mode;
use rustpython_vm::scope::Scope;
use rustpython_vm::Interpreter;
use utils::Debuged;

pub mod cc_mod;
pub mod lua_api;
pub mod utils;

// #[export_name = "export_func"]
// extern "C" fn export_func() {
//     "entry".export();
// }

// #[export_name = "entry"]
// extern "C" fn entry() {
//     // // show_str(&format!("{}",));
//     // 32i32.export();
//     // 42i64.export();
//     // 1.3f32.export();
//     // 1.431f64.export();
//     python();
// }
export_funcs!((python, p), (init, init_python));

thread_local! {
    static I : Interpreter = Interpreter::without_stdlib(Default::default());
    static S:Scope = {
        I.with(|i|i.enter(|vm|vm.new_scope_with_builtins()))
    };
}

fn cal1(a: i32) -> LuaResult<i32> {
    Ok(a * 32)
}
fn python(pystr: Option<String>) -> LuaResult<Option<String>> {
    // let pystr = String::import().unwrap();
    let res = I.with(|e| {
        S.with(|s| {
            e.enter(|vm| {
                if pystr.is_none() {
                    return Ok(None);
                }
                let source = pystr.as_deref().unwrap_or("pass");
                let code_obj = vm.compile(source, Mode::BlockExpr, "<embedded>".to_owned())?;

                let a = vm.run_code_obj(code_obj, s.to_owned()).debuged()?;

                let scope = vm.new_scope_with_builtins();
                scope.locals.set_item("a", a, vm).debuged()?;
                let a = vm.run_block_expr(scope, "str(a)").debuged()?;

                Ok(Some(format!("{}", a.str(vm).debuged()?)))
            })
        })
    });
    res
}
fn init() -> LuaResult<()> {
    S.with(|_| {});
    Ok(())
}
