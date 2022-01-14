use rand::Rng;
use s_home_proto::{DeviceAction, DeviceRequest, Marshal, Response};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread::{sleep, spawn};
use std::time::Duration;

pub struct State {
    is_on: bool,
    power: f32,
}

impl State {
    pub fn new() -> Self {
        Self {
            is_on: false,
            power: 20.0,
        }
    }
}

pub fn serve(mut state: State, port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(address.as_str()).unwrap();
    println!("listening on {}", address);

    spawn(move || {
        let mut rng = rand::thread_rng();
        let mut heartbeat = 0u32;
        loop {
            if heartbeat == 0 || heartbeat % 10 == 0 {
                println!("heartbeat #{}", heartbeat)
            }
            heartbeat += 1;

            state.power += rng.gen_range(-2.0..2.0);
            sleep(Duration::from_secs(1));
        }
    });

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();
        println!("buf: {}", buf.as_str());

        let mut exit_flag = false;

        let req = DeviceRequest::unmarshal(buf.as_str()).unwrap();
        println!("request: {:?}", &req);

        let resp = match req {
            DeviceRequest::Ping => Response::Pong,
            DeviceRequest::Status => Response::Status(state.is_on),
            DeviceRequest::GetTemperature => Response::Power(state.power),
            DeviceRequest::DeviceAction { method } => match method {
                DeviceAction::TurnOn => {
                    state.is_on = true;
                    Response::Ok
                }
                DeviceAction::TurnOff => {
                    state.is_on = false;
                    Response::Ok
                }
            },
            // exiting totally
            DeviceRequest::Exit => {
                exit_flag = true;
                Response::Ok
            }
            _ => Response::Err(format!("bad request: {:?}", req)),
        };
        stream.write_all(resp.marshal().as_bytes()).unwrap();
        // exit totally
        if exit_flag {
            break;
        }
    }
    Ok(())
}
