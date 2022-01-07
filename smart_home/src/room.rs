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
            Some(string_error::into_err(format!(
                "room already contains device '{}'",
                name
            )))
        } else {
            self.devices.insert(name.to_string(), device);
            None
        };
        RoomUpdateResult { err }
    }

    pub fn remove_device(&mut self, name: &str) -> RoomUpdateResult {
        let err = if !self.devices.contains_key(name) {
            Some(string_error::into_err(format!(
                "room does not contain device '{}'",
                name
            )))
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
            let err = Some(string_error::into_err(format!(
                "device '{}' does not exist",
                name
            )));
            Err(RoomReadResult { err })
        }
    }

    pub fn get_summary(&self) -> String {
        let mut out = String::new();

        for device in self.devices.values() {
            let status = device.get_status();
            match status {
                Ok(s) => out.push_str(s.as_string().as_str()),
                Err(err) => out.push_str(format!("error getting status: {}", err).as_str()),
            }
            out.push('\n')
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::power_socket::PowerSocket;
    use crate::devices::thermometer::Thermometer;
    use crate::room::Room;

    static POWER_SOCKET: &str = "poser_socket";
    static THERMOMETER: &str = "thermometer";

    fn new_room() -> Room {
        Room::new("test room")
    }

    fn new_power_socket() -> PowerSocket {
        PowerSocket::new("test socket")
    }

    fn new_thermometer() -> Thermometer {
        Thermometer::new("test thermometer")
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
        todo!()
    }

    #[test]
    fn test_list_devices() {
        todo!()
    }

    #[test]
    fn test_get_device() {
        todo!()
    }

    #[test]
    fn test_get_summary() {
        todo!()
    }
}
