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

    let distro = lua.create_function(|_, ()| {
        Ok(get_os())
    }).unwrap();

    let hostname = lua.create_function(|_, ()| {
        Ok(get_hostname())
    }).unwrap();

    let username = lua.create_function(|_, ()| {
        Ok(get_username())
    }).unwrap();

    yafetch.set("hostname", hostname).unwrap();
    yafetch.set("username", username).unwrap();
    yafetch.set("distro", distro).unwrap();

    globals.set("yafetch", yafetch).unwrap();

    let contents: String = std::fs::read_to_string(config).expect("failed to read init.lua");

    lua.load(&contents).exec().unwrap();

    let init: Function = globals.get("init").unwrap();

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