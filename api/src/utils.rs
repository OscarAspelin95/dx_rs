pub fn time_now() -> String {
    chrono::Local::now()
        .to_utc()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
