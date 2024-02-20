use machine_info::Machine;

pub fn get() -> String {
    let mut m = Machine::new();
    let s = m.system_info();
    return s.processor.brand
}
