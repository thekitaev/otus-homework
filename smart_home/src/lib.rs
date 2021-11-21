#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::Error;

pub struct Home {
    name: String,
    rooms: HashMap<String, Room>,
}

pub struct HomeReadResult {}

pub struct HomeUpdateResult {}

impl Home {
    pub fn add_room(&mut self, _name: String) -> HomeUpdateResult {
        todo!()
    }
    pub fn remove_room(&mut self) -> HomeUpdateResult {
        todo!()
    }
    pub fn list_rooms(&self) -> Vec<Room> {
        todo!()
    }
    pub fn collect_summary(&self) -> String {
        todo!()
    }
}

pub struct RoomReadResult {}

pub struct RoomUpdateResult {}

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
    pub fn get_device(&self, _name: &str) -> Result<Box<dyn Device>, Error> {
        todo!()
    }
    pub fn get_summary(&self) -> String {
        todo!()
    }
}

enum DeviceCondition {
    Ok,
    Err(Error),
    Unknown,
}

pub struct DeviceStatus {
    condition: DeviceCondition,
    status: String,
}

impl DeviceStatus {
    pub fn as_string(&self) -> &str {
        todo!()
    }
}

pub trait Device {
    fn get_status(&self) -> Result<DeviceStatus, Error>;
}

struct DeviceUpdateResult {}

struct Thermometer {
    name: String,
    description: String,
    temp: f64,
}

impl Thermometer {
    fn get_temp(&self) -> f64 {
        todo!()
    }
}

impl Device for Thermometer {
    fn get_status(&self) -> Result<DeviceStatus, Error> {
        todo!()
    }
}

struct PowerSocket {
    name: String,
    description: String,
    power: f64,
    is_on: bool,
}

impl PowerSocket {
    fn power_on(&mut self) -> DeviceUpdateResult {
        todo!()
    }
    fn power_off(&mut self) -> DeviceUpdateResult {
        todo!()
    }
    fn get_power_consumption(&self) -> f64 {
        todo!()
    }
}

impl Device for PowerSocket {
    fn get_status(&self) -> Result<DeviceStatus, Error> {
        todo!()
    }
}
