extern crate rscam;
extern crate serial;
extern crate inter_sense;

use std::path::PathBuf;
use std::process::Command;
use std::{thread, time, fs};
use std::sync::Arc;

const DEFAULT_PATH: &'static str = "frames/";

mod cam;
mod gps;

//tcpdump -s 1248 -i enp2s0 -w test.pcap port 2368
fn record_velodyne(path: &str, interf: &str)  {
    let mut path_buf = PathBuf::from(path);
    path_buf.push("velodyne.pcap");
    Command::new("sudo")
            .arg("tcpdump")
            .arg("-s").arg("1248")
            .arg("-i").arg(interf)
            .arg("-w").arg(path_buf.to_str().unwrap())
            .arg("port").arg("2368")
            .output()
            .expect("failed to execute process");
}

fn record_imu(path: &str) {
    let dt = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    let mut path_buf = PathBuf::from(path);
    let name = format!("imu.{}_{}.bin", dt.as_secs(), dt.subsec_nanos());
    path_buf.push(name);
    inter_sense::record(&path_buf).expect("Failed to record IMU");
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(v) => v,
        None => DEFAULT_PATH.to_string(),
    };

    fs::create_dir_all(&path).unwrap();

    let mut pb = PathBuf::from(&path);
    pb.push("cam1");
    thread::spawn(move|| cam::record(pb, "/dev/video0"));

    let mut pb = PathBuf::from(&path);
    pb.push("cam2");
    thread::spawn(move|| cam::record(pb, "/dev/video1"));

    let path_ref = Arc::new(path);
    let pr = path_ref.clone();
    thread::spawn(move|| gps::record(pr.as_ref(), "/dev/ttyUSB1").expect("Failed to get GPS"));

    let pr = path_ref.clone();
    thread::spawn(move|| record_imu(pr.as_ref()));

    record_velodyne(path_ref.as_ref(), "enp3s0");
}
