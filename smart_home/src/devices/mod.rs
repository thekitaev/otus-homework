use std::error::Error;
use std::fmt::{Display, Formatter};

mod power_socket;
mod thermometer;

enum DeviceCondition {
    Ok,
    Err(Box<dyn Error>),
    Unknown,
}

pub struct DeviceStatus {
    condition: DeviceCondition,
    status: String,
}

impl DeviceStatus {
    pub fn as_string(&self) -> &str {
        todo!()
    }
}

pub trait Device {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>>;
}

#[derive(Debug)]
struct DeviceUpdateResult {}

impl Display for DeviceUpdateResult {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for DeviceUpdateResult {}
