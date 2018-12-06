use rscam::{Camera, Frame, Config};

use std::io::Write;
use std::path::PathBuf;
use std::{io, fs, time};

const CAM_GAIN: u32 = 0;
const CAM_EXPOSURE: u32 = 7500;
const TARG_FPS: u64 = 50; // externall trigger freq

const CAM_W: usize = 2448;
const CAM_H: usize = 2048;
const CAM_FPS: u32 = 60;
const BUF_NUM: usize = 10;

const TRIGGER_DELAY_ID: u32 = 0x0199E210;
const TRIGGER_MODE_ID: u32 = 0x0199E208;
const WB_AUTO_ID: u32 = 0x0098090C;
const WB_RED_ID: u32 = 0x0098090E;
const WB_BLUE_ID: u32 = 0x0098090F;
const WB_GREEN_ID: u32 = 0x0199E248;
const SHUTTER_AUTO_ID: u32 = 0x0199E202;
const EXPOSURE_TIME_US_ID: u32 = 0x0199E201;

const GAIN_ID: u32 = 0x00980913;
const GAIN_AUTO_ID: u32 = 0x0199E205;

const HIGHLIGHT_REDUCTION_ID: u32 = 26862163;

pub fn configure_camera(dev: &str) -> io::Result<()> {
    let cam = Camera::new(dev)?;
    /*
    for control in cam.controls() {
        match control {
            Ok(val) => println!("0x{:08X} {}", val.id, val.name),
            Err(e) => println!("Error: {}", e),
        }
    }
    */

    cam.set_control(TRIGGER_DELAY_ID, 0)?;
    cam.set_control(TRIGGER_MODE_ID, true)?;
    cam.set_control(SHUTTER_AUTO_ID, false)?;
    cam.set_control(EXPOSURE_TIME_US_ID, CAM_EXPOSURE)?;

    cam.set_control(WB_AUTO_ID, false)?;
    cam.set_control(WB_RED_ID, 121i32)?;
    cam.set_control(WB_BLUE_ID, 109i32)?;
    cam.set_control(WB_GREEN_ID, 0i32)?;

    cam.set_control(GAIN_AUTO_ID, false)?;
    cam.set_control(GAIN_ID, CAM_GAIN)?;

    cam.set_control(HIGHLIGHT_REDUCTION_ID, false)?;
    Ok(())
}

fn get_camera(dev: &str) -> io::Result<Camera> {
    println!("conf: {:?}", dev);
    configure_camera(&dev)?;
    println!("conf done: {:?}", dev);

    let mut camera = Camera::new(dev)?;
    camera.start(&Config {
        interval: (1, CAM_FPS),
        resolution: (CAM_W as u32, CAM_H as u32),
        format: b"RGGB",
        nbuffers: BUF_NUM as u32,
        ..Default::default()
    }).expect("Failed to start camera");

    Ok(camera)
}

fn save_frame(frame: &Frame, mut path: PathBuf) -> io::Result<()> {
    let dt = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    let t_abs = 1000*(dt.as_secs() as u64) + (dt.subsec_nanos() as u64)/1_000_000;

    let t = frame.get_timestamp();

    path.push(format!("{}_{}.pnm", t_abs, t/1000));
    let mut f = fs::File::create(path)?;
    f.write_all(b"P5\n2448 2048\n255\n")?;
    f.write_all(&frame)?;
    Ok(())
}

fn check_timing(cam_dev: &str, t: u64, prev: u64) {
    let d = t - prev;
    if d < 900_000/TARG_FPS {
        println!("unexpected frame [{}]: {}", cam_dev, d);
    } else if d > 1_100_000/TARG_FPS {
        println!("frame drop [{}]: {}", cam_dev, d);
    }
}

pub fn record(path: PathBuf, cam_dev: &str) {
    fs::create_dir_all(&path).unwrap();
    let cam = get_camera(cam_dev).expect("Failed to get camera");

    // clear empty frames
    for _ in 0..BUF_NUM {
        let frame = cam.capture().unwrap();
        if frame.len() != 0 {
            println!("Init drop got non-zero frames [{}]: {}",
                cam_dev, frame.len());
        }
    }

    let mut prev = 0;
    loop {
        let frame = cam.capture().unwrap();
        let t = frame.get_timestamp();
        if prev != 0 {
            check_timing(cam_dev, t, prev);
        } else {
            println!("First frame [{}]: {:?}", cam_dev, t);
        }
        prev = t;

        if frame.len() != CAM_W*CAM_H {
            println!("Bad frame len [{}]: {}", cam_dev, frame.len());
            continue
        }
        save_frame(&frame, path.clone()).unwrap();
    }
}
