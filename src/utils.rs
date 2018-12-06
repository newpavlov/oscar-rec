use std::time::SystemTime;

// Returns timestamp in microseconds of UNIX epoch
pub(crate) fn get_timestamp_us() -> u64 {
    let t = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    1_000_000*t.as_secs() + (t.subsec_micros() as u64)
}
