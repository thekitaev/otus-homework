use crate::devices::{
    device_needs_update, make_device_udp_request, Device, DeviceCondition, DeviceStatus,
};
use s_home_proto::{DeviceRequest, Response};
use std::time::Instant;

use super::DeviceReadError;

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

    pub async fn get_temp(&mut self) -> Result<f32, DeviceReadError> {
        if !self.dsn.is_empty() && device_needs_update(self.last_updated) {
            let request_result = make_device_udp_request(&self.dsn, DeviceRequest::GetTemperature).await;
            let resp = if let Err(err) = request_result {
                return Err(DeviceReadError::UnknownError(err));
            } else {
                request_result.unwrap()
            };
            return match resp {
                Response::Temperature(val) => {
                    self.temp = val;
                    self.last_updated = Some(Instant::now());
                    self.condition = DeviceCondition::Ok;
                    Ok(val)
                }
                Response::Err(err_msg) => {
                    self.condition = DeviceCondition::Err(err_msg.to_string());
                    Err(DeviceReadError::ErrMakingRequest(err_msg.to_string()))
                }
                _ => {
                    self.condition = DeviceCondition::Unknown;
                    Err(DeviceReadError::UnexpectedResponse(resp))
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
    use tokio;

    const NAME: &str = "test thermometer";

    fn new_thermometer() -> Thermometer {
        Thermometer::new(NAME, "")
    }

    #[tokio::test]
    async fn test_get_temp() {
        let mut device = new_thermometer();
        let temp = device.get_temp().await.unwrap();
        assert_eq!(temp, 0.0)
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
