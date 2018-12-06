#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::{fs, io, mem, path, thread, time};
use std::io::{Write, BufWriter};

const DATA_SIZE: usize = 444;

// timestamp in microseconds of UNIX epoch
fn get_timestamp_us() -> u64 {
    let t = time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .unwrap();
    1_000_000*t.as_secs() + (t.subsec_micros() as u64)
}

pub fn record<P: AsRef<path::Path>>(path: P) -> io::Result<()> {
    let mut f = BufWriter::new(fs::File::create(path)?);
    unsafe {
        let handle = ISD_OpenTracker(0, 0, 0, 0);
        if handle == -1 {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                "InterSense tracker not present"));
        }

        let sleep_dt = time::Duration::from_micros(100);
        loop {
            let mut data: ISD_TRACKING_DATA_TYPE = mem::uninitialized();
            let res = ISD_GetTrackingData(handle, &mut data);
            if res == -1 {
                Err(io::Error::new(io::ErrorKind::Other,
                    "Failed to get tracking data"))?;
            }

            let data = &data.Station[0];
            if data.NewData != 0 {
                let t = get_timestamp_us();
                let buf: [u8; 8] = mem::transmute(t);
                f.write_all(&buf)?;

                let buf: [u8; DATA_SIZE] = mem::transmute_copy(data);
                f.write_all(&buf)?;
            }

            thread::sleep(sleep_dt);
        }
    }
}
