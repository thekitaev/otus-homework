use smart_home::devices::power_socket::PowerSocket;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_with_mock_server() {
    let arc = PowerSocket::new("test power socket", "", "127.0.0.1:1234");

    let state = power_socket_server::State::new();
    let _server_thread = thread::spawn(move || power_socket_server::serve(state, 1234).unwrap());

    let arc_clone = Arc::clone(&arc);
    let _polling_thread = thread::spawn(move || PowerSocket::start_poll(arc_clone));

    println!("threads started, waiting");
    thread::sleep(Duration::from_secs(1));

    {
        let power_on_result = arc.write().unwrap().power_on();
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
        let read_guard = arc.read().unwrap();
        power_consumption = read_guard.get_power_consumption();
        println!("power_consumption = {}", power_consumption);
        assert_ne!(power_consumption, 0.0);
        // exiting scope to drop the lock
    }
    {
        thread::sleep(Duration::from_secs(1));
        let read_guard = arc.read().unwrap();
        let new_power_consumption = read_guard.get_power_consumption();
        println!("new_power_consumption = {}", new_power_consumption);
        assert_ne!(new_power_consumption, power_consumption);
    }
}
