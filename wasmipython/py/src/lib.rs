use ffi::{export_str, import_string};
use rustpython_vm::compiler::Mode;
use rustpython_vm::scope::Scope;
use rustpython_vm::Interpreter;
use utils::SyncNonsyncLazy;

mod ffi;
mod utils;
// thread_local! {
//     static I : Interpreter = Interpreter::without_stdlib(Default::default());
//     static S:Scope = {
//         I.with(|i|i.enter(|vm|vm.new_scope_with_builtins()))
//     };

// }
static I: SyncNonsyncLazy<Interpreter> =
    SyncNonsyncLazy::new(|| Interpreter::without_stdlib(Default::default()));
static S: SyncNonsyncLazy<Scope> =
    SyncNonsyncLazy::new(|| I.enter(|vm| vm.new_scope_with_builtins()));

#[no_mangle]
pub extern "C" fn a() {
    export_str(&(import_string() + "aaaaa"));
}

// fn eval_(pystr: Option<String>) -> ! {
//     // let pystr = String::import().unwrap();
//     let res = I.with(|e| {
//         S.with(|s| {
//             e.enter(|vm| {
//                 if pystr.is_none() {
//                     return Ok(None);
//                 }
//                 let source = pystr.as_deref().unwrap_or("pass");
//                 let code_obj = vm.compile(source, Mode::BlockExpr, "<embedded>".to_owned())?;

//                 let a = vm.run_code_obj(code_obj, s.to_owned()).debuged()?;

//                 let scope = vm.new_scope_with_builtins();
//                 scope.locals.set_item("a", a, vm).debuged()?;
//                 let a = vm.run_block_expr(scope, "str(a)").debuged()?;

//                 Ok(Some(format!("{}", a.str(vm).debuged()?)))
//             })
//         })
//     });
//     res
// }
// fn exec_(pystr: Option<String>) -> LuaResult<()> {
//     // let pystr = String::import().unwrap();
//     let res = I.with(|e| {
//         S.with(|s| {
//             e.enter(|vm| {
//                 if pystr.is_none() {
//                     return Ok(());
//                 }
//                 let source = pystr.as_deref().unwrap_or("pass");
//                 let code_obj = vm.compile(source, Mode::Exec, "<embedded>".to_owned())?;

//                 vm.run_code_obj(code_obj, s.to_owned()).debuged()?;

//                 Ok(())
//             })
//         })
//     });
//     res
// }
// fn init_() -> LuaResult<()> {
//     S.with(|_| {});
//     Ok(())
// }
