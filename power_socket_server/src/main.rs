use rand::Rng;
use s_home_proto::{DeviceAction, DeviceRequest, Marshal, Response};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread::{sleep, spawn};
use std::time::Duration;

struct State {
    is_on: bool,
    power: f32,
}

fn main() {
    let mut state = State {
        is_on: false,
        power: 0.0,
    };

    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    spawn(move || {
        let mut rng = rand::thread_rng();

        loop {
            state.power = rng.gen_range(10.0..20.0);
            sleep(Duration::from_secs(1))
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
                Response::Status(format!("status: {}, power: {:.1}", status, power))
            }
            DeviceRequest::DeviceAction { method } => {
                match method {
                    DeviceAction::TurnOn => {
                        state.is_on = true;
                        Response::Ok
                    }
                    DeviceAction::TurnOff => {
                        state.is_on = false;
                        Response::Ok
                    }
                }
            }
        };
        stream.write_all(resp.marshal().as_bytes()).unwrap();
    }
}
