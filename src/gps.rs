use serial;
use serial::SerialPort;
use std::io;
use std::fs::File;
use std::io::{Write, BufRead, BufWriter, BufReader};
use std::path::Path;
use std::time::{Duration, SystemTime};

// timestamp in microseconds of UNIX epoch
fn get_timestamp_us() -> u64 {
    let t = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    1_000_000*t.as_secs() + (t.subsec_micros() as u64)
}

pub fn record(path: &Path, port: &str) -> io::Result<()> {
    let path = path.join("gps.log");
    let mut file = BufWriter::new(File::create(path)?);

    let mut port = serial::open(port)?;
    let settings = serial::PortSettings {
        baud_rate: serial::BaudRate::from_speed(9600),
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    };
    port.configure(&settings)?;
    port.set_timeout(Duration::from_secs(5))?;

    let mut br = BufReader::new(&mut port);
    let mut buf = String::with_capacity(255);
    let _ = br.read_line(&mut buf);
    loop {
        br.read_line(&mut buf)?;
        write!(file, "{}\t", get_timestamp_us())?;
        //println!("{:?}", buf);
        file.write_all(buf.as_bytes())?;
        buf.clear();
    }
}
