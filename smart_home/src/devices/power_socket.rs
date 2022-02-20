use crate::devices::{
    device_needs_update, make_device_tcp_request, Device, DeviceCondition, DeviceStatus,
    DeviceUpdateResult,
};
use s_home_proto::{DeviceAction, DeviceRequest, Response};
use std::error::Error;
use std::time::Instant;

static DEVICE_NAME: &str = "PSOC";

pub struct PowerSocket {
    name: String,
    dsn: String,
    power: f32,
    is_on: bool,
    condition: DeviceCondition,
    last_updated: Option<Instant>,
}

impl PowerSocket {
    pub fn new(name: &str, dsn: &str) -> Self {
        let condition = if dsn.is_empty() {
            DeviceCondition::Ok
        } else {
            DeviceCondition::Unknown
        };
        Self {
            name: name.to_string(),
            dsn: dsn.to_string(),
            power: 0.0,
            is_on: false,
            last_updated: None,
            condition,
        }
    }

    pub async fn power_on(&mut self) -> DeviceUpdateResult {
        self.set_power(true).await
    }

    pub async fn power_off(&mut self) -> DeviceUpdateResult {
        self.set_power(false).await
    }

    async fn set_power(&mut self, state: bool) -> DeviceUpdateResult {
        if self.dsn.is_empty() {
            self.is_on = state;
            return DeviceUpdateResult::new(None);
        }

        let method = if state {
            DeviceAction::TurnOn
        } else {
            DeviceAction::TurnOff
        };

        let req = DeviceRequest::DeviceAction { method };

        let result = make_device_tcp_request(self.dsn.as_str(), req).await;
        let err = match result {
            Err(err) => Some(err),
            Ok(resp) => match resp {
                Response::Ok => {
                    self.is_on = state;
                    None
                }
                _ => {
                    let msg = format!("unexpected response: {:?}", resp);
                    eprintln!("{}", msg);
                    Some(msg.into())
                }
            },
        };
        DeviceUpdateResult::new(err)
    }

    pub async fn get_power_consumption(&mut self) -> Result<f32, Box<dyn Error>> {
        if !self.dsn.is_empty() && device_needs_update(self.last_updated) {
            let resp = make_device_tcp_request(&self.dsn, DeviceRequest::GetPower).await?;
            return match resp {
                Response::Power(val) => {
                    self.power = val;
                    self.last_updated = Some(Instant::now());
                    self.condition = DeviceCondition::Ok;
                    Ok(val)
                }
                Response::Err(err_msg) => {
                    self.condition = DeviceCondition::Err(err_msg.to_string());
                    Err(format!("err requesting power: {}", err_msg).into())
                }
                _ => {
                    self.condition = DeviceCondition::Unknown;
                    Err(format!("unexpected response: {:?}", resp).into())
                }
            };
        }
        Ok(self.power)
    }
}

impl Device for PowerSocket {
    fn get_status(&self) -> DeviceStatus {
        DeviceStatus {
            device_type: DEVICE_NAME.to_string(),
            name: self.name.to_string(),
            condition: self.condition.clone(),
            status: format!("power: {}", self.power),
            updated: self.last_updated,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::power_socket::{PowerSocket, DEVICE_NAME};
    use crate::devices::{Device, DeviceCondition, DeviceStatus};
    use tokio;

    const NAME: &str = "test power socket";

    fn new_power_socket() -> PowerSocket {
        PowerSocket::new(NAME, "")
    }

    #[tokio::test]
    async fn test_power_on() {
        let mut device = new_power_socket();
        if let Some(err) = device.power_on().await.err {
            panic!("{err}")
        }
    }

    #[tokio::test]
    async fn test_power_off() {
        let mut device = new_power_socket();
        if let Some(err) = device.power_off().await.err {
            panic!("{err}")
        }
    }

    #[tokio::test]
    async fn test_get_power_consumption() {
        let mut device = new_power_socket();
        assert_eq!(device.get_power_consumption().await.unwrap(), 0.0)
    }

    #[test]
    fn test_get_status() {
        let device = new_power_socket();

        let have = device.get_status();
        let mut want = DeviceStatus {
            device_type: DEVICE_NAME.to_string(),
            name: NAME.to_string(),
            condition: DeviceCondition::Ok,
            status: format!("power: {}", 0.0),
            updated: None,
        };
        assert_eq!(have, want);

        // testing Eq trait works :-)
        want.status = "changed".to_string();
        assert_ne!(have, want)
    }
}
