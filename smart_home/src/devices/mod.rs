use std::fmt::Error;

mod power_socket;
mod thermometer;

enum DeviceCondition {
    Ok,
    Err(Error),
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
    fn get_status(&self) -> Result<DeviceStatus, Error>;
}

struct DeviceUpdateResult {}
