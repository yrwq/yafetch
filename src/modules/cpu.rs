use machine_info::Machine;

/// Returns the cpu brand
pub fn get() -> String {
    let mut m = Machine::new();
    let s = m.system_info();
    return s.processor.brand
}
