use power_socket_server::{serve, State};
use s_home_proto::{DeviceRequest, Marshal, Response};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

#[test]
fn test_request() {
    let state = State::new();

    thread::spawn(|| {
        serve(state, 1234).unwrap();
    });
    const CLIENT_PREFIX: &str = "[CLIENT]";

    println!("{} server thread started, waiting", CLIENT_PREFIX);

    thread::sleep(Duration::from_secs(1));
    let mut stream = std::net::TcpStream::connect("127.0.0.1:1234").unwrap();
    println!("{} connected", CLIENT_PREFIX);
    stream
        .write(DeviceRequest::Ping.marshal().as_bytes())
        .unwrap();
    println!("{} packet written", CLIENT_PREFIX);

    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    println!("{} packet read", CLIENT_PREFIX);

    let resp = Response::unmarshal(buf.as_str()).unwrap();
    assert_eq!(resp, Response::Pong)
}
