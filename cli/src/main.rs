use mlua::{Function, Lua, Variadic};
use whoami;
use std::process::Command;
mod helpers;

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


fn get_music() -> String {
    let data = Command::new("playerctl")
        .arg("metadata")
        .arg("--format")
        .arg("{{ xesam:artist }} - {{ xesam:title }}") // leave the extra "a" there
        .output().unwrap();
     let music = String::from_utf8_lossy(&data.stdout)
        .into_owned()
        .split('\n')
        .collect::<Vec<&str>>()[0]
        .to_string();
    return music;
}


fn run(config: String) {
    let lua = Lua::new();
    let globals = lua.globals();

    let yafetch = lua.create_table().unwrap();

    let header = lua.create_function(|_, strings: Variadic<String>| {
        Ok(print!("{:>14} @ {}\n", 
        get_hostname(),
        get_username(),
    ))
    }).unwrap();

    let format = lua.create_function(|_, strings: Variadic<String>| {
        Ok(print!("{:>12}   :   {}\n", 
        strings[0],
        strings[1],
    ))
    }).unwrap();

    let distro = lua.create_function(|_, ()| {
        Ok(get_os())
    }).unwrap();

    let hostname = lua.create_function(|_, ()| {
        Ok(get_hostname())
    }).unwrap();

    let username = lua.create_function(|_, ()| {
        Ok(get_username())
    }).unwrap();

    let music = lua.create_function(|_, ()| {
        Ok(get_music())
    }).unwrap();

    yafetch.set("hostname", hostname).unwrap();
    yafetch.set("username", username).unwrap();
    yafetch.set("distro", distro).unwrap();
    yafetch.set("music", music).unwrap();
    yafetch.set("format", format).unwrap();
    yafetch.set("header", header).unwrap();

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
    println!("");
    run(config);
    println!("");
}
