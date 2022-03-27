use s_home_proto::{DeviceRequest, Marshal, Response};
use std::fmt::{write, Debug, Display, Formatter};
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::io;
use tokio::net::{TcpStream, UdpSocket};

pub mod power_socket;
pub mod thermometer;

static UPDATE_INTERVAL: Duration = Duration::from_millis(500);

fn device_needs_update(updated: Option<Instant>) -> bool {
    updated.is_none() || updated.unwrap().lt(&(Instant::now() - UPDATE_INTERVAL))
}

#[derive(Eq, PartialEq)]
enum DeviceCondition {
    Ok,
    Err(String),
    Unknown,
}

impl Clone for DeviceCondition {
    fn clone(&self) -> Self {
        match self {
            Self::Ok => Self::Ok,
            Self::Unknown => Self::Unknown,
            Self::Err(err) => Self::Err(err.to_string()),
        }
    }
}

impl Display for DeviceCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::Err(err) => write!(f, "ERROR: {}", err),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct DeviceStatus {
    device_type: String,
    name: String,
    condition: DeviceCondition,
    status: String,
    updated: Option<Instant>,
}

impl DeviceStatus {
    pub fn quick_unknown(name: &str, device_type: &str) -> Self {
        Self {
            device_type: device_type.to_string(),
            name: name.to_string(),
            condition: DeviceCondition::Unknown,
            status: "UNKNOWN".to_string(),
            updated: None,
        }
    }

    pub fn as_string(&self) -> String {
        format!(
            "[{}]{}\n\tcondition: {}\n\tstatus: {}\n",
            self.device_type.to_uppercase(),
            self.name,
            self.condition,
            self.status
        )
    }

    fn as_compact_string(&self) -> String {
        let updated = if self.updated.is_none() {
            "NEVER".to_string()
        } else {
            format!("{:.1}", self.updated.unwrap().elapsed().as_secs_f32())
        };
        format!(
            "[{}]{}-{}*{} UPD: {}",
            self.condition, self.device_type, self.name, self.status, updated
        )
    }
}

impl Debug for DeviceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}", self.as_compact_string()))
    }
}

pub trait Device {
    fn get_status(&self) -> DeviceStatus;
    // fn start_poll(&mut self);
}

type DeviceRequestResult = Result<s_home_proto::Response, Box<dyn std::error::Error>>;

pub(crate) async fn make_device_tcp_request(dsn: &str, req: DeviceRequest) -> DeviceRequestResult {
    println!("[TCP FUNC] making request: {:?}", &req);

    let stream = TcpStream::connect(dsn).await?;
    let msg = req.marshal()?;
    let bytes_written = stream.try_write(msg.as_bytes())?;
    println!("[TCP FUNC] WRITTEN {} bytes", bytes_written);

    let mut buf = Vec::with_capacity(512);
    loop {
        stream.readable().await?;

        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => println!("[TCP FUNC] READ {} bytes", n),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                eprintln!("got error @ try_read_buf: {}", &e);
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }

    let resp = Response::unmarshal(std::str::from_utf8(&buf)?).unwrap();
    Ok(resp)
}

pub(crate) async fn make_device_udp_request(dsn: &str, req: DeviceRequest) -> DeviceRequestResult {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    socket.connect(dsn).await?;
    socket.send(req.marshal().unwrap().as_bytes()).await?;

    let mut buf = [0u8; 512];
    let bytes_read = socket.recv(&mut buf).await?;

    let msg = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
    let resp = Response::unmarshal(msg.as_str())?;
    Ok(resp)
}

#[derive(Error, Debug)]
pub enum DeviceUpdateError {
    #[error("unexpected response")]
    UnexpectedResponse(s_home_proto::Response),
    #[error("unknown error: {0}")]
    UnknownError(Box<dyn std::error::Error>),
}

#[derive(Error, Debug)]
pub enum DeviceReadError {
    #[error("unexpected response")]
    UnexpectedResponse(s_home_proto::Response),
    #[error("err making request: {0}")]
    ErrMakingRequest(String),
    #[error("unknown error: {0}")]
    UnknownError(Box<dyn std::error::Error>),
}
