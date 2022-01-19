use crate::devices::{
    device_needs_update, make_device_udp_request, Device, DeviceCondition, DeviceStatus,
};
use s_home_proto::{DeviceRequest, Response};
use std::error::Error;
use std::time::Instant;

static DEVICE_NAME: &str = "THRM";

pub struct Thermometer {
    name: String,
    dsn: String,
    temp: f32,
    condition: DeviceCondition,
    last_updated: Option<Instant>,
}

impl Thermometer {
    pub fn new(name: &str, dsn: &str) -> Self {
        Self {
            name: name.to_string(),
            dsn: dsn.to_string(),
            temp: 0.0,
            condition: if dsn.is_empty() {
                DeviceCondition::Ok
            } else {
                DeviceCondition::Unknown
            },
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
                    self.condition = DeviceCondition::Ok;
                    Ok(val)
                }
                Response::Err(err_msg) => {
                    self.condition = DeviceCondition::Err(err_msg.to_string());
                    Err(format!("err requesting temperature: {}", err_msg).into())
                }
                _ => {
                    self.condition = DeviceCondition::Unknown;
                    Err(format!("unexpected response: {:?}", resp).into())
                }
            };
        }
        Ok(self.temp)
    }
}

impl Device for Thermometer {
    fn get_status(&self) -> DeviceStatus {
        DeviceStatus {
            device_type: DEVICE_NAME.to_string(),
            name: self.name.to_string(),
            condition: self.condition.clone(),
            status: format!("temperature: {}", self.temp),
            updated: self.last_updated,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::thermometer::{Thermometer, DEVICE_NAME};
    use crate::devices::{Device, DeviceCondition, DeviceStatus};
    const NAME: &str = "test thermometer";

    fn new_thermometer() -> Thermometer {
        Thermometer::new(NAME, "")
    }

    #[test]
    fn test_get_temp() {
        let mut device = new_thermometer();
        assert_eq!(device.get_temp().unwrap(), 0.0)
    }

    #[test]
    fn test_get_status() {
        let device = new_thermometer();

        let have = device.get_status();
        let mut want = DeviceStatus {
            device_type: DEVICE_NAME.to_string(),
            name: NAME.to_string(),
            condition: DeviceCondition::Ok,
            status: format!("temperature: {}", 0.0),
            updated: None,
        };
        assert_eq!(have, want);

        want.status = "changed".to_string();
        assert_ne!(have, want)
    }
}
