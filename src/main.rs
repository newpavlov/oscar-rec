extern crate rscam;
extern crate serial;
extern crate inter_sense;
extern crate structopt;
#[macro_use] extern crate structopt_derive;

use std::path::Path;
use std::process::Command;
use std::{thread, time, fs, mem};

use structopt::StructOpt;

mod cam;
mod gps;
mod cli;

//tcpdump -s 1248 -i enp2s0 -w test.pcap port 2368
fn record_velodyne(path: &Path, interf: &str)  {
    let path = path.join("velodyne.pcap");
    Command::new("sudo")
        .arg("tcpdump")
        .arg("-s").arg("1248")
        .arg("-i").arg(interf)
        .arg("-w").arg(path)
        .arg("port").arg("2368")
        .output()
        .expect("failed to execute process");
}

// timestamp in microseconds of UNIX epoch
fn get_timestamp_us() -> u64 {
    let t = time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .unwrap();
    1_000_000*t.as_secs() + (t.subsec_micros() as u64)
}

fn record_imu(path: &Path) {
    let name = format!("imu.{}.bin", get_timestamp_us());
    inter_sense::record(&path.join(name)).expect("Failed to record IMU");
}

fn to_static<T: Sized>(s: T) -> &'static T {
    let ret = unsafe { mem::transmute(&s as &T) };
    mem::forget(s);
    ret
}

fn main() {
    let args = to_static(cli::Cli::from_args());

    fs::create_dir_all(&args.path).unwrap();

    let path1 = args.path.join("cam1");
    thread::spawn(move|| cam::record(path1, "/dev/video0", &args.cam_ctrls));

    let path2 = args.path.join("cam2");
    thread::spawn(move|| cam::record(path2, "/dev/video1", &args.cam_ctrls));

    thread::spawn(move|| gps::record(&args.path, "/dev/ttyUSB1").expect("Failed to get GPS"));

    thread::spawn(move|| record_imu(&args.path));

    record_velodyne(&args.path, "enp3s0");
}
