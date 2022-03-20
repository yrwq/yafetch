use mlua::{Function, Lua, Table};
mod script;

fn main() {
    let lua = Lua::new();
    let globals = lua.globals();

    let config = script::get_config();
    let contents = std::fs::read_to_string(config).expect("failed to read init.lua");

    lua.load(&contents).exec().unwrap();

    let name: Table = globals.get("yafetch").unwrap();
    let init: Function = name.get("init").unwrap();

    init.call::<_, ()>("").unwrap();
}

