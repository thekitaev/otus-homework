use rand::Rng;
use s_home_proto::{DeviceAction, DeviceRequest, Marshal, Response};
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

struct State {
    is_on: bool,
    temp: f32,
}

pub fn serve(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let state = State {
        is_on: true,
        temp: 20.0,
    };
    let arc = Arc::new(Mutex::new(state));
    let arc_clone = Arc::clone(&arc);

    spawn(move || {
        let mut rng = rand::thread_rng();

        loop {
            let mut state = arc_clone.lock().unwrap();
            state.temp += rng.gen_range(0.005..0.05);
            println!("[SERVER] temperature changed: new value {}", state.temp);
            drop(state);
            sleep(Duration::from_secs(1))
        }
    });

    let socket = UdpSocket::bind(addr).unwrap();
    let mut buf = [0; 1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((recv, addr)) => {
                let s = String::from_utf8_lossy(&buf[..recv]).to_string();
                println!("[SERVER] received {} bytes", recv);
                println!("[SERVER] message = {}", s);

                let mut state = arc.lock().unwrap();

                let req =
                    DeviceRequest::unmarshal(s.as_str()).expect("err unmarshalling device request");
                let resp = match req {
                    DeviceRequest::Ping => Response::Pong,
                    DeviceRequest::Status => Response::Status(state.is_on),
                    DeviceRequest::DeviceAction { method } => {
                        match method {
                            DeviceAction::TurnOff => state.is_on = false,
                            DeviceAction::TurnOn => state.is_on = true,
                        };
                        Response::Ok
                    }
                    DeviceRequest::GetTemperature => Response::Temperature(state.temp),
                    _ => Response::Err(format!("bad request: {:?}", req)),
                };
                let message = resp.marshal().unwrap();
                socket.send_to(message.as_bytes(), addr).unwrap();
            }
            Err(e) => println!("err receiving a datagram: {}", &e),
        }
    }
}
