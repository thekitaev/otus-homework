use crate::devices::{Device, DeviceStatus};
use std::error::Error;

struct Thermometer {
    name: String,
    description: String,
    dsn: String,
    temp: f64,
    last_updated: Option<std::time::Instant>,
}

impl Thermometer {
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
