use serde::{Deserialize, Serialize};

pub trait Marshal {
    fn marshal(&self) -> String
        where
            Self: Serialize,
    {
        let result = serde_json::to_string(&self).unwrap();
        println!("marshal produced: {}", &result);
        result
    }
    fn unmarshal<'a>(s: &'a str) -> serde_json::Result<Self>
        where
            Self: Sized,
            Self: Deserialize<'a>,
    {
        println!("unmarshal got str: {}", s);
        serde_json::from_str(s)
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
    HomeAction {
        method: HomeAction,
        room_name: String,
    },
}

impl Marshal for HomeRequest {}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DeviceRequest {
    Ping,
    Status,
    DeviceAction { method: DeviceAction },
    GetTemperature,
    GetPower,
    Exit,
}

impl Marshal for DeviceRequest {}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Response {
    Pong,
    Ok,
    Err(String),
    Status(bool),
    Temperature(f32),
    Power(f32),
}

impl Marshal for Response {}

#[cfg(test)]
mod tests {
    use crate::{DeviceRequest, Marshal};

    #[test]
    fn test_marshal() {
        let req = DeviceRequest::Ping;
        let bs = req.marshal();
        println!("marshal result: {}", bs);

        let new_req = DeviceRequest::unmarshal(&bs).unwrap();
        assert_eq!(req, new_req);
        println!("test marshal success")
    }
}
