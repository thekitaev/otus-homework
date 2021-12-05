use crate::devices::Device;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct RoomReadResult {}

impl Display for RoomReadResult {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for RoomReadResult {}

#[derive(Debug)]
pub struct RoomUpdateResult {}

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
    pub fn add_device(&mut self, _name: String, _device: Box<dyn Device>) -> RoomUpdateResult {
        todo!()
    }
    pub fn remove_device(&mut self, _name: &str) -> RoomUpdateResult {
        todo!()
    }
    pub fn list_devices(&self) -> Vec<Box<dyn Device>> {
        todo!()
    }
    pub fn get_device(&self, _name: &str) -> Result<Box<dyn Device>, RoomReadResult> {
        todo!()
    }
    pub fn get_summary(&self) -> String {
        todo!()
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
