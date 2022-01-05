use crate::room::Room;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct Home {
    name: String,
    rooms: HashMap<String, Room>,
}

impl Home {
    pub fn new(name: &str) -> Home {
        Home {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct HomeReadResult {
    err: Option<Box<dyn Error>>,
}

impl Display for HomeReadResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.err {
            Some(err) => write!(f, "read failed: {}", err),
            None => Ok(()),
        }
    }
}

impl Error for HomeReadResult {}

#[derive(Debug)]
pub struct HomeUpdateResult {
    err: Option<Box<dyn Error>>,
}

impl Display for HomeUpdateResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.err {
            Some(err) => write!(f, "update failed: {}", err),
            None => Ok(()),
        }
    }
}

impl Error for HomeUpdateResult {}

impl Home {
    pub fn add_room(&mut self, name: &str) -> HomeUpdateResult {
        let err = if self.rooms.contains_key(name) {
            Some(string_error::into_err(format!(
                "home already contains room '{}'",
                name
            )))
        } else {
            self.rooms.insert(name.to_string(), Room::new(name));
            None
        };
        HomeUpdateResult { err }
    }

    pub fn get_room(&mut self, name: &str) -> Result<&Room, HomeReadResult> {
        if self.rooms.contains_key(name) {
            Ok(self.rooms.get(name).unwrap())
        } else {
            Err(HomeReadResult {
                err: Some(string_error::into_err(format!(
                    "home does not contain room '{}'",
                    name
                ))),
            })
        }
    }

    pub fn remove_room(&mut self, name: &str) -> HomeUpdateResult {
        let err = if !self.rooms.contains_key(name) {
            Some(string_error::into_err(format!(
                "home does not contain room '{}'",
                name
            )))
        } else {
            self.rooms.remove(name);
            None
        };
        HomeUpdateResult { err }
    }

    pub fn list_rooms(&self) -> Vec<&Room> {
        let mut out = vec![];

        for room in self.rooms.values() {
            out.push(room);
        }
        out
    }

    pub fn collect_summary(&self) -> String {
        let mut out = String::new();

        for room in self.rooms.values() {
            out.push_str(room.get_summary().as_str())
        }
        out
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_room() {
        todo!()
    }

    #[test]
    fn test_get_room() {
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
