use std::net::UdpSocket;
use rand::Rng;
use s_home_proto::{DeviceAction, DeviceRequest, Marshal, Response};
use std::thread::{sleep, spawn};
use std::time::Duration;

struct State {
    is_on: bool,
    temp: f32,
}

fn main() {
    let mut state = State{
        is_on: true,
        temp: 20.0,
    };

    spawn(move || {
        let mut rng = rand::thread_rng();

        loop {
            state.temp += rng.gen_range(0.005..0.05);
            sleep(Duration::from_secs(1))
        }
    });

    let socket = UdpSocket::bind("127.0.0.1:1235").unwrap();
    let mut buf = [0;1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((_amt, _src)) => {
                let s = String::from_utf8_lossy(&buf).to_string();
                let req = DeviceRequest::unmarshal(&s).unwrap();
                let resp = match req {
                   DeviceRequest::Ping => Response::Pong,
                   DeviceRequest::Status => {
                       let (status, temp) = match state.is_on {
                           true => ("on", state.temp),
                           false => ("off", 0.0),
                       };
                       Response::Status(format!("status: {}, temp: {:.1}", status, temp))
                   } 
                   DeviceRequest::DeviceAction {method}=> {
                       match method {
                            DeviceAction::TurnOff => state.is_on = false,
                            DeviceAction::TurnOn => state.is_on = true    
                       };
                       Response::Ok
                   }
                };
                let message = resp.marshal();
                socket.send(message.as_bytes().as_ref()).unwrap();
            },
            Err(e) => println!("err receiving a datagram: {}", &e)
            
        }
    }
    
}
