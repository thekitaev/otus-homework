use crate::devices::Device;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct RoomReadResult {
    err: Option<Box<dyn Error>>,
}

impl Display for RoomReadResult {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for RoomReadResult {}

#[derive(Debug)]
pub struct RoomUpdateResult {
    err: Option<Box<dyn Error>>,
}

impl Display for RoomUpdateResult {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for RoomUpdateResult {}

pub struct Room {
    name: String,
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
        let mut err = None;

        if self.devices.contains_key(name) {
            err = Some(string_error::into_err(format!(
                "room already contains device '{}'",
                name
            )));
        } else {
            self.devices.insert(name.to_string(), device);
        }
        RoomUpdateResult { err }
    }

    pub fn remove_device(&mut self, name: &str) -> RoomUpdateResult {
        let mut err = None;

        if !self.devices.contains_key(name) {
            err = Some(string_error::into_err(format!(
                "room does not contain device '{}'",
                name
            )))
        } else {
            self.devices.remove(name);
        }
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
                Ok(s) => out.push_str(s.as_string()),
                Err(err) => out.push_str(format!("error getting status: {}", err).as_str()),
            }
            out.push('\n')
        }

        out
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_device() {
        todo!()
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
