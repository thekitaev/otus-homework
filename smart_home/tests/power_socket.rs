use smart_home::devices::power_socket::PowerSocket;
use std::thread;
use std::time::Duration;

#[test]
fn test_with_mock_server() {
    let mut device = PowerSocket::new("test power socket", "127.0.0.1:1234");

    let state = power_socket_server::State::new();
    let _server_thread = thread::spawn(move || power_socket_server::serve(state, 1234).unwrap());

    println!("thread started, waiting");
    thread::sleep(Duration::from_secs(1));

    {
        let power_on_result = device.power_on();
        if let Some(err) = power_on_result.err {
            panic!("err powering on: {}", err)
        } else {
            println!("powering on OK")
        }
    }
    // above scope for comparison
    let power_consumption: f32;
    {
        // waiting for server to change value and a poller to receive it
        thread::sleep(Duration::from_secs(1));
        power_consumption = device.get_power_consumption().unwrap();
        println!("power_consumption = {}", power_consumption);
        assert_ne!(power_consumption, 0.0);
        // exiting scope to drop the lock
    }
    {
        thread::sleep(Duration::from_secs(1));
        let new_power_consumption = device.get_power_consumption().unwrap();
        println!("new_power_consumption = {}", new_power_consumption);
        assert_ne!(new_power_consumption, power_consumption);
    }
}
