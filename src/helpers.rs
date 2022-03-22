pub fn get_config() -> String {
    let mut home_cfg: String = std::env::var("HOME").unwrap();
    home_cfg.push_str("/.config");

    let mut config_location = std::env::var("XDG_CONFIG_HOME").unwrap_or(home_cfg);

    config_location.push_str("/yafetch/init.lua");

    return config_location;
}
