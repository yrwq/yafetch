use std::fs;

/// Returns the cpu vendor and brand
pub fn get() -> String {
    let mut path = "/sys/devices/virtual/dmi/id/product_name";
    if fs::metadata(path).is_err() {
        path = "/sys/firmware/devicetree/base/model";
    }

    let f = fs::read_to_string(path);
    return f.unwrap();
}
