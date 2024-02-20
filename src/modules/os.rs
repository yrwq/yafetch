use sysinfo::System;

pub fn get() -> String {
    return System::name().unwrap();
}
