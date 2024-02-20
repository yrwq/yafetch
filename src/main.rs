pub mod modules;
mod yafetch;
use mlua::Lua;

use machine_info::Machine;

/// try to get the user's config
fn get_config() -> std::path::PathBuf {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("yafetch").unwrap();
    let config_path = xdg_dirs.find_config_file("init.lua")
        .expect("could not find init.lua in ~/.config/yafetch");
    return config_path;
}

fn main() {
    // initialize yafetch with a lua state
    let yf = yafetch::Yafetch{
        lua: Lua::new(),
    };

    // register functions
    yf.register();


    let args: Vec<String> = std::env::args().collect(); 

    match args.len() {
        // no argument passed
        1 => {
            yf.run(get_config());
        }
        2 => {
            yf.run(args[1].clone().into());
        }
        _ => {
            // TODO: help message
        }
    }
}
