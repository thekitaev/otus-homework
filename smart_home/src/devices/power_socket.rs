use crate::devices::{
    device_needs_update, make_device_tcp_request, Device, DeviceCondition, DeviceStatus,
    DeviceUpdateResult,
};
use s_home_proto::{DeviceAction, DeviceRequest, Response};
use std::error::Error;
use std::time::Instant;

pub struct PowerSocket {
    name: String,
    dsn: String,
    power: f32,
    is_on: bool,
    last_updated: Option<Instant>,
}

impl PowerSocket {
    pub fn new(name: &str, dsn: &str) -> Self {
        Self {
            name: name.to_string(),
            dsn: dsn.to_string(),
            power: 0.0,
            is_on: false,
            last_updated: None,
        }
    }

    pub fn power_on(&mut self) -> DeviceUpdateResult {
        self.set_power(true)
    }

    pub fn power_off(&mut self) -> DeviceUpdateResult {
        self.set_power(false)
    }

    fn set_power(&mut self, state: bool) -> DeviceUpdateResult {
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

        let result = make_device_tcp_request(self.dsn.as_str(), req);
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

    pub fn get_power_consumption(&mut self) -> Result<f32, Box<dyn Error>> {
        if !self.dsn.is_empty() && device_needs_update(self.last_updated) {
            let resp = make_device_tcp_request(&self.dsn, DeviceRequest::GetPower).unwrap();
            return match resp {
                Response::Power(val) => {
                    self.power = val;
                    self.last_updated = Some(Instant::now());
                    Ok(val)
                }
                Response::Err(err) => Err(format!("err requesting power: {}", err).into()),
                _ => Err(format!("unexpected response: {:?}", resp).into()),
            };
        }
        Ok(self.power)
    }
}

impl Device for PowerSocket {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>> {
        let device_type = "Power socket";

        Ok(DeviceStatus {
            device_type: device_type.to_string(),
            name: self.name.to_string(),
            condition: DeviceCondition::Ok,
            status: format!("power: {}", self.power),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::power_socket::PowerSocket;

    extern crate power_socket_server;

    fn new_power_socket() -> PowerSocket {
        PowerSocket::new("test power socket", "")
    }

    #[test]
    fn test_power_on() {
        let mut device = new_power_socket();
        if let Some(err) = device.power_on().err {
            panic!("{}", err)
        }
    }

    #[test]
    fn test_power_off() {
        let mut device = new_power_socket();
        if let Some(err) = device.power_off().err {
            panic!("{}", err)
        }
    }

    #[test]
    fn test_get_power_consumption() {
        let mut device = new_power_socket();
        assert_eq!(device.get_power_consumption().unwrap(), 0.0)
    }

    #[test]
    fn test_get_status() {
        let _device = new_power_socket();
        // TODO: make test
    }
}
