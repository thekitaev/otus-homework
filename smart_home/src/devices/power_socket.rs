use crate::devices::{Device, DeviceCondition, DeviceUpdateResult};

struct PowerSocket {
    name: String,
    description: String,
    power: f64,
    is_on: bool,
}

impl PowerSocket {
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
    fn get_status(&self) -> DeviceCondition {
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
