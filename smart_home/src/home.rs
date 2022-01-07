use crate::quick_display_and_error;
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

quick_display_and_error!(HomeReadResult);

#[derive(Debug)]
pub struct HomeUpdateResult {
    err: Option<Box<dyn Error>>,
}
quick_display_and_error!(HomeUpdateResult);

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
    use crate::home::Home;

    static KITCHEN: &str = "kitchen";

    fn new_home() -> Home {
        Home::new("test home")
    }

    #[test]
    fn test_add_room() {
        let mut home = new_home();

        let add_room_ok = home.add_room(KITCHEN);
        match add_room_ok.err {
            Some(err) => panic!("err {} should not be present", err),
            _ => {}
        }

        let add_room_err = home.add_room(KITCHEN);
        match add_room_err.err {
            Some(_err) => {}
            None => panic!("err should be present now, but it does not"),
        }
    }

    #[test]
    fn test_get_room() {
        let mut home = new_home();
        home.add_room(KITCHEN);

        let get_kitchen_ok = home.get_room(KITCHEN);
        match get_kitchen_ok {
            Ok(res) => {
                if res.name != KITCHEN {
                    panic!("get_room: result_name {} wrong", res.name)
                }
            }
            Err(err) => panic!("get_room test failed: err getting room: {}", err),
        }
    }

    #[test]
    fn test_remove_room() {
        let mut home = new_home();
        home.add_room(KITCHEN);

        let remove_kitchen_ok = home.remove_room(KITCHEN);
        match remove_kitchen_ok.err {
            Some(err) => panic!("remove_room failed: err {}", err),
            _ => {}
        }
    }

    #[test]
    fn test_list_rooms() {
        let mut home = new_home();
        let rooms_names = vec!["room_1", "room_2"];
        for room in rooms_names {
            home.add_room(room);
        }

        let rooms = home.list_rooms();
        assert_eq!(2, rooms.len())
    }

    #[test]
    fn test_collect_summary() {
        todo!()
    }
}
