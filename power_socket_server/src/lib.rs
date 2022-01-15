use std::error::Error;
use s_home_proto::{DeviceAction, DeviceRequest, Marshal, Response};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
use rand::Rng;

static SERVER_PREFIX: &str = "[SERVER]";

pub struct State {
    is_on: bool,
    power: f32,
}

impl State {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            is_on: false,
            power: 20.0,
        }))
    }
}

pub fn serve(state: Arc<Mutex<State>>, port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(address.as_str()).unwrap();
    println!(
        "{} listening on {} with log level {}",
        SERVER_PREFIX,
        address,
        log::max_level()
    );

    let  state_clone = Arc::clone(&state);
    spawn(move || {
        let mut rng = rand::thread_rng();
        let mut heartbeat = 0u32;
        loop {
            if heartbeat == 0 || heartbeat % 10 == 0 {
                println!("{} heartbeat #{}", SERVER_PREFIX, heartbeat)
            }
            heartbeat += 1;

            state_clone.lock().unwrap().power += rng.gen_range(-2.0f32..2.0f32);
            sleep(Duration::from_secs(1));
        }
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let state = Arc::clone(&state);

        let result = handle_connection(stream, state);
        match result {
            Ok(true) => break,
            Ok(false) => {
                println!("{} request handled", SERVER_PREFIX)
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: std::net::TcpStream, state: Arc<Mutex<State>>) -> Result<bool, Box<dyn Error>> {
    let mut buf = [0u8; 200];
    let message_len = stream.read(&mut buf).unwrap();
    println!("{} buf: {}", SERVER_PREFIX, message_len);

    let mut exit_flag = false;

    let bs = String::from_utf8_lossy(&buf).to_string();
    println!("{} message: {}", SERVER_PREFIX, bs.as_str());

    let req = DeviceRequest::unmarshal(bs.as_str()).unwrap().into();
    println!("{} request: {:?}", SERVER_PREFIX, &req);

    let mut state = state.lock().unwrap();

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
        return Ok(true);
    }
    Ok(false)
}