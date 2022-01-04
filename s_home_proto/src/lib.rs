use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

pub trait Marshal {
    fn marshal(&self) -> String where Self: Serialize {
        to_string(&self).unwrap()
    }
    fn unmarshal<'a>(s: &'a str) -> serde_json::Result<Self> where Self: Sized, Self: Deserialize<'a> {
        from_str(s)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceAction {
    TurnOn,
    TurnOff,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum HomeAction {
    AddRoom,
    RemoveRoom,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum HomeRequest {
    Ping,
    Status,
    HomeAction { method: HomeAction, room_name: String },
}

impl Marshal for HomeRequest {}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceRequest {
    Ping,
    Status,
    DeviceAction { method: DeviceAction },
}

impl Marshal for DeviceRequest {}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Response {
    Pong,
    Ok,
    Err(String),
    Status(String),
}

impl Marshal for Response {}

#[cfg(test)]
mod tests {
    use crate::{HomeRequest, Marshal};

    #[test]
    fn test_marshal() {
        let req = HomeRequest::Ping;
        let bs = req.marshal();
        let new_req = HomeRequest::unmarshal(&bs).unwrap();
        assert_eq!(req, new_req);
        println!("test marshal success")
    }
}