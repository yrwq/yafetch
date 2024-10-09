use sysinfo::{CpuRefreshKind, RefreshKind, System};

/// Returns the cpu vendor and brand
pub fn get() -> String {
    let s = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    let cpu = s.cpus().first();
    let brand = cpu.unwrap().brand().to_string();
    let vendor = cpu.unwrap().vendor_id().to_string();
    let result = format!("{vendor} {brand}");
    return result;
}
