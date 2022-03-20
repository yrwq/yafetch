use mlua::{Function, Lua, Table};
mod helpers;

fn run(config: String) {
    let lua = Lua::new();
    let globals = lua.globals();

    let contents = std::fs::read_to_string(config).expect("failed to read init.lua");

    lua.load(&contents).exec().unwrap();

    let name: Table = globals.get("yafetch").unwrap();
    let init: Function = name.get("init").unwrap();

    init.call::<_, ()>("").unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config: String;
    if args.len() > 1 {
        config = String::from(&args[1]);
    } else {
        config = helpers::get_config();
    }

    run(config);
}
