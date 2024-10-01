use cc_wasm_api::export_funcs;
use cc_wasm_api::lua_api::{next_import_type, Exportable, Importable, LuaError, LuaResult, Typed};
use cc_wasm_api::utils::Debuged;
use rustpython_vm::compiler::Mode;
use rustpython_vm::scope::Scope;
use rustpython_vm::Interpreter;

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
export_funcs!((eval_, eval), (init_, init), (exec_, exec));

// export_funcs!((mian, a));

fn mian() -> LuaResult<()> {
    // let a = unsafe { import_obj() };
    // unsafe { export_obj(a) };
    let a = LuaObj::import()?;
    a.export();
    Ok(())
}

#[allow(unused)]
#[link(wasm_import_module = "host")]
extern "C" {
    fn export_obj(handle: i32);
    fn drop_obj(handle: i32);
    fn import_obj() -> i32;
}
struct LuaObj {
    handle: i32,
}
impl Importable for LuaObj {
    fn import() -> LuaResult<Self> {
        if next_import_type() != Typed::Object {
            return Err(LuaError::from_str("not receiving Object"));
        }
        Ok(Self {
            handle: unsafe { import_obj() },
        })
    }
}
impl Drop for LuaObj {
    fn drop(&mut self) {
        unsafe { drop_obj(self.handle) };
    }
}
impl Exportable for LuaObj {
    fn export(&self) {
        unsafe {
            export_obj(self.handle);
        }
    }
}

thread_local! {
    static I : Interpreter = Interpreter::without_stdlib(Default::default());
    static S:Scope = {
        I.with(|i|i.enter(|vm|vm.new_scope_with_builtins()))
    };
}

fn panic_() -> LuaResult<()> {
    panic!("intentially paniced")
}
fn multi_return() -> LuaResult<(i32, String, Option<String>, i32)> {
    Ok((2, "21d".into(), None, 212))
}
fn eval_(pystr: Option<String>) -> LuaResult<Option<String>> {
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
fn exec_(pystr: Option<String>) -> LuaResult<()> {
    // let pystr = String::import().unwrap();
    let res = I.with(|e| {
        S.with(|s| {
            e.enter(|vm| {
                if pystr.is_none() {
                    return Ok(());
                }
                let source = pystr.as_deref().unwrap_or("pass");
                let code_obj = vm.compile(source, Mode::Exec, "<embedded>".to_owned())?;

                vm.run_code_obj(code_obj, s.to_owned()).debuged()?;

                Ok(())
            })
        })
    });
    res
}
fn init_() -> LuaResult<()> {
    S.with(|_| {});
    Ok(())
}
