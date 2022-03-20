pub fn get_config() -> String {
    let mut config_location: String;

    let name = "XDG_CONFIG_HOME";
    let mut home: String = std::env::var("HOME").unwrap();
    home.push_str("/.config");

    match std::env::var(name) {
        Ok(v) => config_location = v,
        Err(_e) => config_location = home
    }

    config_location.push_str("/yafetch/init.lua");

    return config_location;
}
