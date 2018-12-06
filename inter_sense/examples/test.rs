extern crate inter_sense;

use std::time;

//sudo ln -s /dev/ttyUSB0 /dev/ttyS4
fn main() {
    let dt = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    println!("{:?}", dt);
    let dt_s = dt.as_secs();
    let dt_ns = dt.subsec_nanos();
    inter_sense::record(format!("imu.{}_{}.bin", dt_s, dt_ns)).unwrap();
}

/*
Euler 4*3
Quaternion 4*4
TimeStamp 4
StillTime 4
CompassYaw 4
AngularVelBodyFrame 4*3 113
AngularVelNavFrame 4*3
AccelBodyFrame 4*3
AccelNavFrame 4*3
AngularVelRaw 4*3
TimeStampSeconds 4
TimeStampMicroSec 4
MagBodyFrame 4*3
*/