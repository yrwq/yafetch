use sysinfo::System;

pub fn get() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();

    return System::kernel_version().unwrap()
}
