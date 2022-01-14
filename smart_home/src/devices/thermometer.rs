use crate::devices::{Device, DeviceCondition, DeviceStatus};
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Thermometer {
    name: String,
    description: String,
    dsn: String,
    temp: f64,
    last_updated: Option<std::time::Instant>,
    rx: Option<thread::JoinHandle<()>>,
}

impl Thermometer {
    pub(crate) fn new(name: &str) -> Arc<RwLock<Self>> {
        let thermometer = Self {
            name: name.to_string(),
            description: "".to_string(),
            dsn: "127.0.0.1:12345".to_string(),
            temp: 0.0,
            last_updated: None,
            rx: None,
        };
        Arc::new(RwLock::new(thermometer))
    }

    fn get_temp(&self) -> f64 {
        self.temp
    }

    fn start_poll(mx: Arc<RwLock<Self>>) {
        let read_guard = mx.read().unwrap();
        // let dsn = read_guard.dsn.clone();
        std::mem::drop(read_guard);

        // let mx_clone = Arc::clone(&mx);

        let jh = thread::spawn(move || {
            println!("starting polling for thermometer");

            // let dsn = dsn.as_str();
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
        Thermometer::new("test thermometer")
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
