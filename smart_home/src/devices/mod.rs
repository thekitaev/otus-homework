use std::error::Error;
use std::fmt::{Display, Formatter};

mod power_socket;
mod thermometer;

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
    condition: DeviceCondition,
    status: String,
}

impl DeviceStatus {
    pub fn as_string(&self) -> String {
        format!("condition: {}, status: {}", self.condition, self.status)
    }
}

pub trait Device {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>>;
}

#[derive(Debug)]
struct DeviceUpdateResult {
    err: Option<Box<dyn Error>>,
}

impl Display for DeviceUpdateResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.err {
            Some(err) => write!(f, "{}", err.to_string()),
            None => write!(f, "OK"),
        }
    }
}

impl Error for DeviceUpdateResult {}
