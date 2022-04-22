use mlua::{Function, Lua};
use whoami;
mod helpers;

// Functions
fn get_hostname() -> String {
    let hname = whoami::hostname();
    return hname;
}

fn get_username() -> String {
    let uname = whoami::username();
    return uname;
}

fn get_os() -> String {
    let os = whoami::distro();
    return os;
}

fn run(config: String) {
    let lua = Lua::new();
    let globals = lua.globals();

    let yafetch = lua.create_table().unwrap();

    yafetch.set("hostname", get_hostname()).unwrap();
    yafetch.set("username", get_username()).unwrap();
    yafetch.set("distro", get_os()).unwrap();

    globals.set("yafetch", yafetch).unwrap();

    let contents: String = std::fs::read_to_string(config).expect("failed to read init.lua");

    lua.load(&contents).exec().unwrap();

    let init: Function = globals.get("init").unwrap();
    // let init: Function = name.get("init").unwrap();

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