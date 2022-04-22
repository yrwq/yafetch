use directories::BaseDirs;
use std::env::consts;

pub fn get_config() -> String {
    if let Some(base_dirs) = BaseDirs::new() {
        let b = base_dirs.config_dir();
        let mut cfg_str = b.display().to_string();
        match consts::OS {
            "windows" => cfg_str.push_str("\\yafetch\\init.lua"),
            _ => cfg_str.push_str("/yafetch/init.lua"),
        }
        return cfg_str;
    } else {
        // hopefully the program will not reach this point
        return "".to_string();
    }
}
