use crate::devices::{Device, DeviceCondition, DeviceStatus};
use std::error::Error;
use std::net::UdpSocket;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use s_home_proto::{DeviceRequest, Marshal, Response};

pub struct Thermometer {
    name: String,
    description: String,
    dsn: String,
    temp: f32,
    last_updated: Option<std::time::Instant>,
    rx: Option<thread::JoinHandle<()>>,
}

impl Thermometer {
    pub fn new(name: &str, description: &str, dsn: &str) -> Arc<RwLock<Self>> {
        let thermometer = Self {
            name: name.to_string(),
            description: description.to_string(),
            dsn: dsn.to_string(),
            temp: 0.0,
            last_updated: None,
            rx: None,
        };
        Arc::new(RwLock::new(thermometer))
    }

    pub fn get_temp(&self) -> f32 {
        self.temp
    }

    pub fn start_poll(mx: Arc<RwLock<Self>>) {
        let read_guard = mx.read().unwrap();
        let dsn = read_guard.dsn.clone();
        std::mem::drop(read_guard);

        let mx_clone = Arc::clone(&mx);

        let jh = thread::spawn(move || {
            println!("starting polling for thermometer");

            loop {
                let dsn = dsn.as_str();
                let socket = UdpSocket::bind("127.0.0.1:1222").unwrap();

                socket.connect(dsn).unwrap();
                socket.send(DeviceRequest::GetTemperature.marshal().unwrap().as_bytes()).unwrap();

                let mut buf = [0u8; 512];
                let bytes_read = socket.recv(&mut buf).unwrap();

                let msg = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
                let resp = Response::unmarshal(msg.as_str()).unwrap();

                match resp {
                    Response::Temperature(temp) => {
                        println!("[CLIENT] temp changed to {}", temp);
                        mx_clone.write().unwrap().temp = temp;
                        println!("[CLIENT] saved new temp")
                    }
                    _ => eprintln!("unexpected response: {:?}", resp)
                }

                thread::sleep(Duration::from_secs(1));
            }
        });

        mx.write().unwrap().rx = Some(jh)
    }
}

impl Device for Arc<RwLock<Thermometer>> {
    fn get_status(&self) -> Result<DeviceStatus, Box<dyn Error>> {
        let guard = self.read().unwrap();
        let device_type = "Thermometer";
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
            status: format!("temperature: {}", guard.temp),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::thermometer::Thermometer;
    use std::sync::{Arc, RwLock};

    fn new_thermometer() -> Arc<RwLock<Thermometer>> {
        Thermometer::new("test thermometer", "", "127.0.0.1:1234")
    }

    #[test]
    fn test_get_temp() {
        let arc = new_thermometer();
        let lock = arc.read().unwrap();
        assert_eq!(lock.get_temp(), 0.0)
    }

    #[test]
    fn test_get_status() {
        let arc = new_thermometer();
        let _guard = arc.read().unwrap();
        // TODO: make test
    }
}
