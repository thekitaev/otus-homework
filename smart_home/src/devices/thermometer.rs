use crate::devices::{Device, DeviceStatus};
use std::error::Error;

pub struct Thermometer {
    name: String,
    description: String,
    dsn: String,
    temp: f64,
    last_updated: Option<std::time::Instant>,
}

impl Thermometer {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: "".to_string(),
            dsn: "127.0.0.1:12345".to_string(),
            temp: 0.0,
            last_updated: None,
        }
    }
    fn get_temp(&self) -> f64 {
        todo!()
    }
}

impl Device for Thermometer {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_temp() {
        todo!()
    }

    #[test]
    fn test_get_status() {
        todo!()
    }
}
