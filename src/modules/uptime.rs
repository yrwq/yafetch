use systemstat::Platform;

pub fn get() -> String {
    let stat = systemstat::System::new();
    let upt = stat.uptime().unwrap();
    let uptime_seconds = upt.as_secs();


    // Calculate the uptime in hours and minutes respectively
    let uptime_hours = uptime_seconds / (60 * 60);
    let uptime_minutes = (uptime_seconds % (60 * 60)) / 60;
    let mut hstr = uptime_hours.to_string();
    let mut mstr = uptime_minutes.to_string();

    hstr.push_str("h ");
    mstr.push_str("m");
    hstr.push_str(&mstr);

    if uptime_hours == 0 {
        return mstr;
    }

    return hstr;
}
