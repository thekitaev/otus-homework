use power_socket_server::{serve, State};
use s_home_proto::{DeviceRequest, Marshal, Response};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

// #[test]
fn test_request() {
    let state = State::new();

    thread::spawn(|| {
        serve(state, 1234).unwrap();
    });
    println!("server thread started, waiting");

    thread::sleep(Duration::from_secs(1));
    let mut stream = std::net::TcpStream::connect("127.0.0.1:1234").unwrap();
    stream
        .write_all(DeviceRequest::Ping.marshal().as_bytes())
        .unwrap();

    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();

    let resp = Response::unmarshal(buf.as_str()).unwrap();
    assert_eq!(resp, Response::Pong)
}
