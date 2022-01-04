use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use rand::Rng;
use s_home_proto::{DeviceRequest, Marshal, Response, DeviceAction};

struct State {
    is_on: bool,
    power: f32,
}

fn main() {
    let mut state = State { is_on: false, power: 0.0 };

    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    thread::spawn(move ||{
        let mut rng = rand::thread_rng();

        loop {
            state.power = rng.gen::<f32>();
        }
    });

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();

        let req = DeviceRequest::unmarshal(buf.as_str()).unwrap();
        let resp = match req {
            DeviceRequest::Ping => Response::Pong,
            DeviceRequest::Status => {
                let (status, power) = match state.is_on {
                    true => ("on", state.power),
                    false => ("off", 0.0),
                };
                Response::Status(format!("status: {}, power: {}", status, power))
            }
            DeviceRequest::DeviceAction { method: DeviceAction::TurnOn } => {
                state.is_on = true;
                Response::Ok
            }
            DeviceRequest::DeviceAction { method: DeviceAction::TurnOff } => {
                state.is_on = false;
                Response::Ok
            }
        };
        stream.write_all(resp.marshal().as_bytes()).unwrap();
    }
}
