use std::error::Error;
use std::fmt::{Display, Formatter};

mod power_socket;
mod thermometer;

pub enum DeviceCondition {
    Ok(String),
    Err(Box<dyn Error>),
    Unknown,
}

impl DeviceCondition {
    pub fn as_string(&self) -> &str {
        todo!()
    }
}

pub trait Device {
    fn get_status(&self) -> DeviceCondition;
}

#[derive(Debug)]
struct DeviceUpdateResult {}

impl Display for DeviceUpdateResult {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for DeviceUpdateResult {}
