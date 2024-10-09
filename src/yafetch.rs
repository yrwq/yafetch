use mlua::prelude::*;

use crate::modules;

pub struct Yafetch {
    pub lua: Lua,
}

impl Yafetch {
    /// run yafetch with the given configuration file
    pub fn run(&self, path: std::path::PathBuf) {
        let src: String = std::fs::read_to_string(path).unwrap();
        let chnk = self.lua.load(&src);
        match chnk.exec() {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }

    // TODO arch

    fn host(&self) -> mlua::Result<mlua::Function> {
        let x = self
            .lua
            .create_function(|_, _: ()| Ok(modules::host::get()));
        Ok(x?)
    }

    fn cpu(&self) -> mlua::Result<mlua::Function> {
        let x = self.lua.create_function(|_, _: ()| Ok(modules::cpu::get()));
        Ok(x?)
    }

    fn kernel(&self) -> mlua::Result<mlua::Function> {
        let x = self
            .lua
            .create_function(|_, _: ()| Ok(modules::kernel::get()));
        Ok(x?)
    }

    fn os(&self) -> mlua::Result<mlua::Function> {
        let x = self.lua.create_function(|_, _: ()| Ok(modules::os::get()));
        Ok(x?)
    }

    fn uptime(&self) -> mlua::Result<mlua::Function> {
        let x = self
            .lua
            .create_function(|_, _: ()| Ok(modules::uptime::get()));
        Ok(x?)
    }

    fn user(&self) -> mlua::Result<mlua::Function> {
        let x = self
            .lua
            .create_function(|_, _: ()| Ok(modules::user::get()));
        Ok(x?)
    }

    fn hostname(&self) -> mlua::Result<mlua::Function> {
        let x = self
            .lua
            .create_function(|_, _: ()| Ok(modules::hostname::get()));
        Ok(x?)
    }

    fn disk_total(&self) -> mlua::Result<mlua::Function> {
        let x = self
            .lua
            .create_function(|_, path: String| Ok(modules::disk::get_total(path)));
        Ok(x?)
    }

    fn disk_free(&self) -> mlua::Result<mlua::Function> {
        let x = self
            .lua
            .create_function(|_, path: String| Ok(modules::disk::get_free(path)));
        Ok(x?)
    }

    fn mem_used(&self) -> mlua::Result<mlua::Function> {
        let x = self
            .lua
            .create_function(|_, _: ()| Ok(modules::mem::get_used()));
        Ok(x?)
    }

    fn mem_total(&self) -> mlua::Result<mlua::Function> {
        let x = self
            .lua
            .create_function(|_, _: ()| Ok(modules::mem::get_total()));
        Ok(x?)
    }

    pub fn register(&self) {
        let globals = self.lua.globals();
        let exports = self.lua.create_table().unwrap();

        // host

        exports
            .set("host", self.host().unwrap())
            .expect("could not register host function");

        exports
            .set("uptime", self.uptime().unwrap())
            .expect("could not register uptime function");

        exports
            .set("cpu", self.cpu().unwrap())
            .expect("could not register cpu function");

        exports
            .set("user", self.user().unwrap())
            .expect("could not register user function");

        exports
            .set("hostname", self.hostname().unwrap())
            .expect("could not register hostname function");

        exports
            .set("kernel", self.kernel().unwrap())
            .expect("could not register kernel function");

        exports
            .set("os", self.os().unwrap())
            .expect("could not register os function");

        //
        // mem
        //

        exports
            .set("mem_used", self.mem_used().unwrap())
            .expect("could not register mem_used function");

        exports
            .set("mem_total", self.mem_total().unwrap())
            .expect("could not register mem_total function");

        //
        // disk
        //

        exports
            .set("disk_free", self.disk_free().unwrap())
            .expect("could not register disk_free function");

        exports
            .set("disk_total", self.disk_total().unwrap())
            .expect("could not register disk_total function");

        globals.set("yafetch", exports).unwrap();
    }
}
