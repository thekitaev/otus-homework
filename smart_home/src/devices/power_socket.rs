use crate::devices::{
    make_device_tcp_request, Device, DeviceCondition, DeviceStatus, DeviceUpdateResult,
};
use s_home_proto::{DeviceAction, DeviceRequest, Response};
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub struct PowerSocket {
    name: String,
    description: String,
    dsn: String,
    power: f32,
    is_on: bool,
    last_updated: Option<std::time::Instant>,
    rx: Option<thread::JoinHandle<()>>,
}

impl PowerSocket {
    pub fn new(name: &str, description: &str, dsn: &str) -> Arc<RwLock<Self>> {
        let power_socket = Self {
            name: name.to_string(),
            description: description.to_string(),
            dsn: dsn.to_string(),
            power: 0.0,
            is_on: false,
            last_updated: None,
            rx: None,
        };
        Arc::new(RwLock::new(power_socket))
    }

    pub fn power_on(&mut self) -> DeviceUpdateResult {
        self.set_power(true)
    }

    pub fn power_off(&mut self) -> DeviceUpdateResult {
        self.set_power(false)
    }

    fn set_power(&mut self, state: bool) -> DeviceUpdateResult {
        let method = if state {
            DeviceAction::TurnOn
        } else {
            DeviceAction::TurnOff
        };
        let req = DeviceRequest::DeviceAction { method };

        // if polling is not active
        if self.rx.is_none() {
            self.is_on = state;
            return DeviceUpdateResult::new(None);
        };

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

    pub fn get_power_consumption(&self) -> f32 {
        self.power
    }

    // mx = мютекс, по привычке
    pub fn start_poll(mx: Arc<RwLock<Self>>) {
        let read_guard = mx.read().unwrap();
        let dsn = String::from(read_guard.dsn.as_str());
        if read_guard.dsn.is_empty() {
            eprintln!(
                "cannot start a poll for power socket {}: dsn is empty",
                read_guard.name
            );
            return;
        }

        std::mem::drop(read_guard);

        let mx_clone = Arc::clone(&mx);

        let jh: thread::JoinHandle<()> = thread::spawn(move || {
            println!("starting polling for power_socket");

            let dsn = dsn.as_str();
            loop {
                let req = DeviceRequest::GetPower;
                let response = make_device_tcp_request(dsn, req);

                if let Err(err) = response {
                    eprintln!("request error: {}", err);
                    continue;
                }
                let response = response.unwrap();
                match response {
                    Response::Power(power) => {
                        let mut lock = mx_clone.write().unwrap();
                        lock.power = power;
                        lock.last_updated = Some(std::time::Instant::now());
                    }
                    _ => {
                        eprintln!("unexpected response: {:?}", response)
                    }
                }

                thread::sleep(Duration::from_secs(1));
            }
        });
        mx.write().unwrap().rx = Some(jh)
    }
}

impl Device for Arc<RwLock<PowerSocket>> {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>> {
        let guard = self.read().unwrap();
        let device_type = "Power socket";
        if guard.rx.is_none() {
            return Ok(DeviceStatus::quick_unknown(
                guard.name.as_str(),
                device_type,
            ));
        }
        Ok(DeviceStatus {
            device_type: device_type.to_string(),
            name: guard.name.to_string(),
            condition: DeviceCondition::Ok,
            status: format!("power: {}", guard.power),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::power_socket::PowerSocket;
    use std::sync::{Arc, RwLock};

    extern crate power_socket_server;

    fn new_power_socket() -> Arc<RwLock<PowerSocket>> {
        PowerSocket::new("test power socket", "", "127.0.0.1:1234")
    }

    #[test]
    fn test_power_on() {
        let arc = new_power_socket();
        let mut lock = arc.write().unwrap();
        if let Some(err) = lock.power_on().err {
            panic!("{}", err)
        }
    }

    #[test]
    fn test_power_off() {
        let arc = new_power_socket();
        let mut lock = arc.write().unwrap();
        if let Some(err) = lock.power_off().err {
            panic!("{}", err)
        }
    }

    #[test]
    fn test_get_power_consumption() {
        let arc = new_power_socket();
        let guard = arc.read().unwrap();
        assert_eq!(guard.get_power_consumption(), 0.0)
    }
}
