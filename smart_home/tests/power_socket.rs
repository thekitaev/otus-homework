use smart_home::devices::power_socket::PowerSocket;
use std::thread;
use std::time::Duration;

#[tokio::test]
async fn test_power_socket_with_mock_server() {
    let mut device = PowerSocket::new("test power socket", "127.0.0.1:1234");

    let state = power_socket_server::State::new();
    let _server_thread = thread::spawn(move || power_socket_server::serve(state, 1234).unwrap());

    println!("thread started, waiting");
    tokio::time::sleep(Duration::from_secs(1)).await;

    let power_on_result = device.power_on().await;
    if let Some(err) = power_on_result.err {
        panic!("err powering on: {}", err)
    } else {
        println!("powering on OK")
    }

    // waiting for server to change value and a poller to receive it
    tokio::time::sleep(Duration::from_secs(1)).await;
    let power_consumption = device.get_power_consumption().await.unwrap();
    println!("power_consumption = {}", power_consumption);
    assert_ne!(power_consumption, 0.0);

    tokio::time::sleep(Duration::from_secs(1)).await;
    let new_power_consumption = device.get_power_consumption().await.unwrap();
    println!("new_power_consumption = {}", new_power_consumption);
    assert_ne!(new_power_consumption, power_consumption);
}
