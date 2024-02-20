pub fn get() -> String {
    let hostname = std::fs::read_to_string("/etc/hostname")
        .expect("could not read /etc/hostname");
    return hostname.trim().to_string();
}
