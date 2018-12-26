use serial;
use serial::SerialPort;
use std::io;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::path::Path;
use std::time::Duration;

use utils::get_timestamp_us;

pub fn record(path: &Path, port: &Path) -> io::Result<()> {
    let path = path.join("gps.log");
    let mut file = File::create(path)?;

    let mut port = serial::open(port)?;
    port.configure(&serial::PortSettings {
        baud_rate: serial::BaudRate::from_speed(9600),
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    })?;
    port.set_timeout(Duration::from_secs(5))?;

    let mut br = BufReader::new(&mut port);
    let mut buf = String::with_capacity(255);
    let _ = br.read_line(&mut buf);
    loop {
        br.read_line(&mut buf)?;
        write!(file, "{}\t", get_timestamp_us())?;
        file.write_all(buf.as_bytes())?;
        buf.clear();
    }
}
