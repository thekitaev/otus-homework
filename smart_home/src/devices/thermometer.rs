use crate::devices::{
    device_needs_update, make_device_udp_request, Device, DeviceCondition, DeviceStatus,
};
use s_home_proto::{DeviceRequest, Response};
use std::error::Error;
use std::time::Instant;

pub struct Thermometer {
    name: String,
    dsn: String,
    temp: f32,
    last_updated: Option<Instant>,
}

impl Thermometer {
    pub fn new(name: &str, dsn: &str) -> Self {
        Self {
            name: name.to_string(),
            dsn: dsn.to_string(),
            temp: 0.0,
            last_updated: None,
        }
    }

    pub fn get_temp(&mut self) -> Result<f32, Box<dyn Error>> {
        if !self.dsn.is_empty() && device_needs_update(self.last_updated) {
            let resp = make_device_udp_request(&self.dsn, DeviceRequest::GetTemperature).unwrap();
            return match resp {
                Response::Temperature(val) => {
                    self.temp = val;
                    self.last_updated = Some(Instant::now());
                    Ok(val)
                }
                Response::Err(err) => Err(format!("err requesting temperature: {}", err).into()),
                _ => Err(format!("unexpected response: {:?}", resp).into()),
            };
        }
        Ok(self.temp)
    }
}

impl Device for Thermometer {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>> {
        let device_type = "Thermometer";

        Ok(DeviceStatus {
            device_type: device_type.to_string(),
            name: self.name.to_string(),
            condition: DeviceCondition::Ok,
            status: format!("temperature: {}", self.temp),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::thermometer::Thermometer;

    fn new_thermometer() -> Thermometer {
        Thermometer::new("test thermometer", "")
    }

    #[test]
    fn test_get_temp() {
        let mut device = new_thermometer();
        assert_eq!(device.get_temp().unwrap(), 0.0)
    }

    #[test]
    fn test_get_status() {
        let _device = new_thermometer();
        // TODO: make test
    }
}
