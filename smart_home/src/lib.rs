#![allow(dead_code)]

mod devices;
mod room;

use room::Room;
use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_room() {
        todo!()
    }

    #[test]
    fn test_remove_room() {
        todo!()
    }

    #[test]
    fn test_list_rooms() {
        todo!()
    }

    #[test]
    fn test_collect_summary() {
        todo!()
    }
}
