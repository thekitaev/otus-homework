use s_home_proto::{DeviceRequest, Marshal, Response};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use thermometer_server::serve;

fn quick_sleep(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

#[test]
fn test_server() {
    let addr = "127.0.0.1:12345";
    thread::spawn(|| serve(addr).unwrap());
    println!("server started, waiting");

    quick_sleep(1);

    let cli_socket = UdpSocket::bind("127.0.0.1:1234").unwrap();
    cli_socket.connect(addr).expect("connection failed :)");

    let send_and_get = move |req: DeviceRequest| {
        let req = req.marshal().expect("failed to marshal ping");
        let bytes_sent = cli_socket
            .send(req.as_bytes())
            .expect("failed to send ping request");
        println!("[CLIENT] sent {} bytes", bytes_sent);

        let mut buf = [0u8; 512];
        let bytes_read = cli_socket.recv(&mut buf).unwrap();

        let msg = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
        let resp = Response::unmarshal(msg.as_str()).unwrap();
        resp
    };
    let ping_resp = send_and_get(DeviceRequest::Ping);
    assert_eq!(ping_resp, Response::Pong);

    let first_temp: f32;
    let get_temp_resp = send_and_get(DeviceRequest::GetTemperature);
    match get_temp_resp {
        Response::Temperature(temp) => {
            first_temp = temp;
            println!("get_temp success: temp = {}", temp)
        }
        _ => panic!("unexpected response: {:?}", get_temp_resp),
    }

    // check if temperature changed
    quick_sleep(1);
    let second_temp_resp = send_and_get(DeviceRequest::GetTemperature);
    match second_temp_resp {
        Response::Temperature(temp) => {
            assert_ne!(temp, first_temp);
            assert_ne!(temp, 0f32);
        }
        _ => panic!("unexpected response: {:?}", second_temp_resp),
    }
}
