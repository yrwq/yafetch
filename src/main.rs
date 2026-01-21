pub mod modules;
mod yafetch;
use std::{io, path::{PathBuf, Path}};
use mlua::Lua;

fn get_config() -> Result<PathBuf, io::Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("yafetch")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    xdg_dirs
        .find_config_file("init.lua")
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "init.lua not found",
            )
        })
}

fn main() {
    // initialize yafetch with a lua state
    let yf = yafetch::Yafetch { lua: Lua::new() };

    // register functions
    yf.register();

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        // no argument passed
        1 => {
            match get_config() {
                Ok(contents) => { yf.run(contents) },
                Err(e) => eprintln!("failed to read config file: {}", e)
            }
        }
        // arg passed
        2 => {
            let path = Path::new(&args[1]);

            if !path.exists() {
                eprintln!("file does not exist: {}", path.display());
                eprintln!("usage: yafetch [file.lua]");
                return;
            }

            yf.run(path.into());
        }
        _ => {
            eprintln!("usage: {} [file.lua]", args[0])
        }
    }
}
