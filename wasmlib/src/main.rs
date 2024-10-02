use rustpython_vm::compiler::Mode;
use rustpython_vm::Interpreter;
fn main() {
    let e = Interpreter::without_stdlib(Default::default());
    let res = e.enter(|vm| {
        let scope = vm.new_scope_with_builtins();
        let source = include_str!("1.py");
        let code_obj = vm
            .compile(source, Mode::BlockExpr, "<embedded>".to_owned())
            .map_err(|err| vm.new_syntax_error(&err, Some(source)))
            .unwrap();

        // let a = vm.run_block_expr(scope.clone(), source);
        let a = vm.run_code_obj(code_obj, scope).unwrap();

        let scope = vm.new_scope_with_builtins();
        scope.locals.set_item("a", a, vm).unwrap();
        let a = vm.run_block_expr(scope, "str(a)").unwrap();

        format!("here ! {}", a.str(vm).unwrap())
    });
    dbg!(res);
}
