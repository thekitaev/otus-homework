use crate::devices::{make_device_tcp_request, Device, DeviceStatus, DeviceUpdateResult};
use s_home_proto::{DeviceAction, DeviceRequest, Marshal, Response};
use std::error::Error;
use std::io::{Read, Write};
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
    pub(crate) fn new(name: &str) -> Arc<RwLock<Self>> {
        let power_socket = Self {
            name: name.to_string(),
            description: "".to_string(),
            dsn: "".to_string(),
            power: 0.0,
            is_on: false,
            last_updated: None,
            rx: None,
        };

        let out = Arc::new(RwLock::new(power_socket));
        Self::start_poll(out.clone());
        out
    }

    fn power_on(&mut self) -> DeviceUpdateResult {
        let req = DeviceRequest::DeviceAction {
            method: DeviceAction::TurnOn,
        };
        self.set_on(req, true)
    }

    fn power_off(&mut self) -> DeviceUpdateResult {
        let req = DeviceRequest::DeviceAction {
            method: DeviceAction::TurnOff,
        };
        self.set_on(req, false)
    }

    fn set_on(&mut self, req: DeviceRequest, state: bool) -> DeviceUpdateResult {
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
                    println!("{}", msg);
                    Some(string_error::into_err(msg))
                }
            },
        };
        DeviceUpdateResult { err }
    }

    fn get_power_consumption(&self) -> f32 {
        self.power
    }

    // mx = мютекс,по привычке
    fn start_poll(mx: Arc<RwLock<PowerSocket>>) {
        let read_guard = mx.read().unwrap();
        let dsn = String::from(read_guard.dsn.as_str());
        std::mem::drop(read_guard);

        let mx_clone = mx.clone();

        let jh: thread::JoinHandle<()> = thread::spawn(move || {
            println!("starting polling for power_socket");

            let dsn = dsn.as_str();
            loop {
                let mut stream = std::net::TcpStream::connect(dsn).unwrap();

                let req = DeviceRequest::GetPower.marshal();
                stream.write_all(req.as_bytes()).unwrap();

                let mut buf = String::new();
                stream.read_to_string(&mut buf).unwrap();

                let response = Response::unmarshal(buf.as_str()).unwrap();

                match response {
                    Response::Power(power) => {
                        let mut lock = mx_clone.write().unwrap();
                        lock.power = power;
                        lock.last_updated = Some(std::time::Instant::now());
                    }
                    _ => {
                        println!("unexpected response: {:?}", response)
                    }
                }

                thread::sleep(Duration::from_secs(1));
            }
        });
        mx.write().unwrap().rx = Some(jh)
    }
}

impl Device for PowerSocket {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>> {
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
