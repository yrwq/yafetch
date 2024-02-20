use fs2;

pub fn get_total(path: String) -> String {

    let total = fs2::total_space(path).unwrap();

    let total_gb = total / 1024 / 1024 / 1024;
    return total_gb.to_string();
}

pub fn get_free(path: String) -> String {

    let free = fs2::available_space(path).unwrap();
    let free_gb = free / 1024 / 1024 / 1024;

    return free_gb.to_string();
}
