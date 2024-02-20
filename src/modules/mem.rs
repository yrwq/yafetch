use sysinfo::System;
use bytesize::ByteSize;

pub fn get_used() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mem = ByteSize::gb(sys.used_memory()).to_string();
    let mut split_mem = mem.split(" ");
    //
    let total = split_mem.nth(0).unwrap();

    return total.to_string();
}

pub fn get_total() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mem = ByteSize::gb(sys.total_memory()).to_string();
    let mut split_mem = mem.split(" ");
    //
    let total = split_mem.nth(0).unwrap();

    return total.to_string();
}
