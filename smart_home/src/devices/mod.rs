use crate::quick_display_and_error;
use s_home_proto::{DeviceRequest, Marshal, Response};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};

pub mod power_socket;
pub mod thermometer;

enum DeviceCondition {
    Ok,
    Err(Box<dyn Error>),
    Unknown,
}

impl Display for DeviceCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::Err(err) => write!(f, "ERROR: {}", err.to_string()),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

pub struct DeviceStatus {
    device_type: String,
    name: String,
    condition: DeviceCondition,
    status: String,
}

impl DeviceStatus {
    pub fn quick_unknown(name: &str, device_type: &str) -> Self {
        Self {
            device_type: device_type.to_string(),
            name: name.to_string(),
            condition: DeviceCondition::Unknown,
            status: "UNKNOWN".to_string(),
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
}

pub trait Device {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>>;
    // fn start_poll(&mut self);
}

type DeviceTCPResult = Result<s_home_proto::Response, Box<dyn std::error::Error>>;

pub(crate) fn make_device_tcp_request(dsn: &str, req: DeviceRequest) -> DeviceTCPResult {
    let mut stream = std::net::TcpStream::connect(dsn).unwrap();
    stream.write_all(req.marshal().as_bytes()).unwrap();

    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();

    let resp = Response::unmarshal(buf.as_str()).unwrap();
    Ok(resp)
}

#[derive(Debug)]
struct DeviceUpdateResult {
    err: Option<Box<dyn Error>>,
}

impl DeviceUpdateResult {
    pub fn new(err: Option<Box<dyn Error>>) -> Self {
        Self { err }
    }
}

quick_display_and_error!(DeviceUpdateResult);
