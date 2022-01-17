use serde::{Deserialize, Serialize};

pub trait Marshal {
    fn marshal(&self) -> serde_json::Result<String>
    where
        Self: Serialize,
    {
        serde_json::to_string(&self)
    }
    fn unmarshal<'a>(s: &'a str) -> serde_json::Result<Self>
    where
        Self: Sized,
        Self: Deserialize<'a>,
    {
        serde_json::from_str(s)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum DeviceAction {
    TurnOn,
    TurnOff,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum HomeAction {
    AddRoom,
    RemoveRoom,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "home_request")]
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
#[serde(tag = "device_request")]
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
#[serde(tag = "response", content = "value")]
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
    use crate::{DeviceAction, DeviceRequest, HomeAction, HomeRequest, Marshal, Response};

    #[test]
    fn test_marshal_home_requests() {
        let home_requests = vec![
            HomeRequest::Ping,
            HomeRequest::Status,
            HomeRequest::HomeAction {
                method: HomeAction::AddRoom,
                room_name: "test".to_string(),
            },
        ];
        for req in home_requests {
            let bs = req.marshal().unwrap();
            println!("marshal result: {}", bs);

            let new_req = HomeRequest::unmarshal(&bs).unwrap();
            assert_eq!(req, new_req);
            println!("test marshal success")
        }
    }

    #[test]
    fn test_marshal_device_requests() {
        let home_requests = vec![
            DeviceRequest::Ping,
            DeviceRequest::Status,
            DeviceRequest::DeviceAction {
                method: DeviceAction::TurnOn,
            },
            DeviceRequest::GetTemperature,
            DeviceRequest::GetPower,
            DeviceRequest::Exit,
        ];
        for req in home_requests {
            let bs = req.marshal().unwrap();
            println!("marshal result: {}", bs);

            let new_req = DeviceRequest::unmarshal(&bs).unwrap();
            assert_eq!(req, new_req);
            println!("test marshal success")
        }
    }

    #[test]
    fn test_marshal_responses() {
        let responses = vec![
            Response::Ok,
            Response::Pong,
            Response::Status(true),
            Response::Power(1.2),
            Response::Temperature(5.0),
            Response::Err("something".to_string()),
        ];
        for req in responses {
            let bs = req.marshal().unwrap();
            println!("marshal result: {}", bs);

            let new_req = Response::unmarshal(&bs).unwrap();
            assert_eq!(req, new_req);
            println!("test marshal success")
        }
    }
}
