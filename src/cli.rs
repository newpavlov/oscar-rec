use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(
    name = "oscar-rec",
    about = "Data recording tool for OS:Car project")]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    /// Path to camera device
    pub path: PathBuf,
    #[structopt(flatten)]
    pub cam_ctrls: CamControls
}

#[derive(StructOpt)]
pub struct CamControls {
    #[structopt(long = "exposure", short = "e", default_value="5000")]
    /// Cameras exposure in microseconds
    pub exposure: u32,
    #[structopt(long = "gain", short = "g", default_value="0")]
    /// Camera gain
    pub gain: u32,
}