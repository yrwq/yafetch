pub mod modules;
mod yafetch;
use mlua::Lua;

use machine_info::Machine;

fn main() {
    // initialize yafetch with a lua state
    let yf = yafetch::Yafetch{
        lua: Lua::new(),
    };


    let mut m = Machine::new();
    let s = m.system_info();

    // register functions
    yf.register();

    // run file as script
    // yf.init("/home/yrwq/.config/yafetch/init.lua".to_string());
    yf.init("init.lua".to_string());
}
