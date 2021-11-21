use crate::devices::{Device, DeviceStatus};
use std::fmt::Error;

struct Thermometer {
    name: String,
    description: String,
    temp: f64,
}

impl Thermometer {
    fn get_temp(&self) -> f64 {
        todo!()
    }
}

impl Device for Thermometer {
    fn get_status(&self) -> Result<DeviceStatus, Error> {
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
