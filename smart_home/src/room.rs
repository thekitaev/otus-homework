use crate::devices::Device;
use crate::quick_display_and_error;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct RoomReadResult {
    err: Option<Box<dyn Error>>,
}

quick_display_and_error!(RoomReadResult);

#[derive(Debug)]
pub struct RoomUpdateResult {
    err: Option<Box<dyn Error>>,
}

quick_display_and_error!(RoomUpdateResult);

pub struct Room {
    pub(crate) name: String,
    devices: HashMap<String, Box<dyn Device>>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: HashMap::new(),
        }
    }
    pub fn add_device(&mut self, name: &str, device: Box<dyn Device>) -> RoomUpdateResult {
        let err = if self.devices.contains_key(name) {
            Some(format!("room already contains device '{}'", name).into())
        } else {
            self.devices.insert(name.to_string(), device);
            None
        };
        RoomUpdateResult { err }
    }

    pub fn remove_device(&mut self, name: &str) -> RoomUpdateResult {
        let err = if !self.devices.contains_key(name) {
            Some(format!("room does not contain device '{}'", name).into())
        } else {
            self.devices.remove(name);
            None
        };
        RoomUpdateResult { err }
    }

    pub fn list_devices(&self) -> Vec<&dyn Device> {
        let mut devices = vec![];

        for d in self.devices.values() {
            devices.push(d.as_ref())
        }

        devices
    }

    pub fn get_device(&self, name: &str) -> Result<&dyn Device, RoomReadResult> {
        if self.devices.contains_key(name) {
            let device = self.devices.get(name).unwrap();
            Ok(device.as_ref())
        } else {
            let err = Some(format!("device '{}' does not exist", name).into());
            Err(RoomReadResult { err })
        }
    }

    pub fn get_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(format!("ROOM '{}' SUMMARY:\n", &self.name).as_str());

        if self.devices.is_empty() {
            out.push_str("\t* no devices *")
        }

        for device in self.devices.values() {
            let status = device.get_status();
            match status {
                Ok(s) => out.push_str(s.as_string().as_str()),
                Err(err) => out.push_str(format!("error getting status: {}", err).as_str()),
            }
            out.push('\n')
        }
        out.push('\n');

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::power_socket::PowerSocket;
    use crate::devices::thermometer::Thermometer;
    use crate::room::Room;
    use std::sync::{Arc, RwLock};

    static POWER_SOCKET: &str = "poser_socket";
    static THERMOMETER: &str = "thermometer";
    static TEST_ROOM: &str = "test room";

    fn new_room() -> Room {
        Room::new(TEST_ROOM)
    }

    fn new_power_socket() -> Arc<RwLock<PowerSocket>> {
        PowerSocket::new("test socket", "", "127.0.0.1:1234")
    }

    fn new_thermometer() -> Arc<RwLock<Thermometer>> {
        Thermometer::new("test thermometer", "", "127.0.0.1:12345")
    }

    #[test]
    fn test_add_device() {
        let mut room = new_room();

        // TODO: make a for loop with builder fn and error checking
        let add_power_socket_ok = room.add_device(POWER_SOCKET, Box::new(new_power_socket()));
        if let Some(err) = add_power_socket_ok.err {
            panic!("add_device failed: err {}", err)
        };

        let add_power_socket_err = room.add_device(POWER_SOCKET, Box::new(new_power_socket()));
        if let None = add_power_socket_err.err {
            panic!("add_device must have an err at this point")
        };

        let add_thermometer_ok = room.add_device(THERMOMETER, Box::new(new_thermometer()));
        if let Some(err) = add_thermometer_ok.err {
            panic!("add_device failed: err {}", err)
        };

        let add_thermometer_err = room.add_device(THERMOMETER, Box::new(new_thermometer()));
        if let None = add_thermometer_err.err {
            panic!("add_device must have an err at this point")
        };
    }

    #[test]
    fn test_remove_device() {
        let mut room = new_room();
        room.add_device(THERMOMETER, Box::new(new_thermometer()));

        let remove_device_ok = room.remove_device(THERMOMETER);
        if let Some(err) = remove_device_ok.err {
            panic!("remove_device failed: err {}", err)
        }

        let remove_device_err = room.remove_device(THERMOMETER);
        if let None = remove_device_err.err {
            panic!("remove_device must have an err at this point")
        };
    }

    #[test]
    fn test_list_devices() {
        let mut room = new_room();
        room.add_device(THERMOMETER, Box::new(new_thermometer()));
        room.add_device(POWER_SOCKET, Box::new(new_power_socket()));

        let devices_list = room.list_devices();
        assert_eq!(devices_list.len(), 2)
    }

    #[test]
    fn test_get_device() {
        let mut room = new_room();
        room.add_device(THERMOMETER, Box::new(new_thermometer()));

        let get_device_ok = room.get_device(THERMOMETER);
        match get_device_ok {
            Ok(_res) => println!("get_device OK"),
            Err(err) => panic!("get_device err: {}", err),
        }

        let get_device_err = room.get_device(POWER_SOCKET);
        if let Ok(_res) = get_device_err {
            panic!("get_device must have an err at this point")
        }
    }

    #[test]
    fn test_get_summary() {
        let blank_summary = format!("ROOM '{}' SUMMARY:\n\t* no devices *\n", TEST_ROOM);
        let mut room = new_room();
        assert_eq!(room.get_summary(), blank_summary);

        room.add_device(THERMOMETER, Box::new(new_thermometer()));
        assert_ne!(room.get_summary(), blank_summary);
    }
}
