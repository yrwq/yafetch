pub fn get() -> String {
    let hostname = gethostname::gethostname();
    return hostname.to_str().unwrap().to_string();
}
