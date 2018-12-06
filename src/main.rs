extern crate rscam;
extern crate serial;
extern crate inter_sense;
extern crate structopt;
#[macro_use] extern crate structopt_derive;

use std::path::Path;
use std::process::Command;
use std::{thread, fs};

use structopt::StructOpt;

mod cam;
mod gps;
mod imu;
mod cli;
mod utils;

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

fn to_static<T: Sized>(s: T) -> &'static T {
    Box::leak(Box::new(s))
}

fn main() {
    let args = to_static(cli::Cli::from_args());

    fs::create_dir_all(&args.path).unwrap();

    thread::Builder::new().name("cam1".into())
        .spawn(move|| {
            let path = args.path.join("cam1");
            cam::record(path, "/dev/video0", &args.cam_ctrls)
                .expect("Failed to record camera 1")
        }).unwrap();

    thread::Builder::new().name("cam2".into())
        .spawn(move|| {
            let path = args.path.join("cam2");
            cam::record(path, "/dev/video1", &args.cam_ctrls)
                .expect("Failed to record camera 2")
        }).unwrap();

    thread::Builder::new().name("gps".into())
        .spawn(move|| {
            gps::record(&args.path, &args.gps_path)
                .expect("failed to record GPS")
        }).unwrap();

    thread::Builder::new().name("imu".into())
        .spawn(move|| {
            imu::record(&args.path).expect("failed to record IMU")
        }).unwrap();

    record_velodyne(&args.path, "enp3s0");
}
