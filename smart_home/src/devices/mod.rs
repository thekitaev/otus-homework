use crate::quick_display_and_error;
use std::error::Error;
use std::fmt::{Display, Formatter};

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
}

#[derive(Debug)]
struct DeviceUpdateResult {
    err: Option<Box<dyn Error>>,
}

quick_display_and_error!(DeviceUpdateResult);
