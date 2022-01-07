use crate::devices::{Device, DeviceStatus, DeviceUpdateResult};
use std::error::Error;

pub struct PowerSocket {
    name: String,
    description: String,
    dsn: String,
    power: f64,
    is_on: bool,
    last_updated: Option<std::time::Instant>,
}

impl PowerSocket {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: "".to_string(),
            dsn: "".to_string(),
            power: 0.0,
            is_on: false,
            last_updated: None,
        }
    }
    fn power_on(&mut self) -> DeviceUpdateResult {
        todo!()
    }
    fn power_off(&mut self) -> DeviceUpdateResult {
        todo!()
    }
    fn get_power_consumption(&self) -> f64 {
        todo!()
    }
}

impl Device for PowerSocket {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_power_on() {
        todo!()
    }

    #[test]
    fn test_power_off() {
        todo!()
    }

    #[test]
    fn test_get_power_consumption() {
        todo!()
    }

    #[test]
    fn test_get_status() {
        todo!()
    }
}
