use rlua::Lua;

fn main()  {
    let lua = Lua::new();

    lua.context(|ctx| {
        let globals = ctx.globals();

        globals.set("string_var", "hello").unwrap();
        globals.set("int_var", 42).unwrap();
        let i: i32 = globals.get("int_var").unwrap();
        println!("{}", i)
    });
}
