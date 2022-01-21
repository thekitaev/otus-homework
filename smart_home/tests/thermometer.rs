use smart_home::devices::thermometer::Thermometer;
use std::thread;
use std::time::Duration;

#[tokio::test]
async fn test_thermometer_with_mock_server() {
    let mut device = Thermometer::new("test power socket", "127.0.0.1:1234");

    thread::spawn(|| thermometer_server::serve("127.0.0.1:1234").unwrap());

    println!("thread started, waiting");
    thread::sleep(Duration::from_secs(1));

    let temp = device.get_temp().await.unwrap();
    println!("temp = {}", temp);
    assert_ne!(temp, 20.0, "temp should not be default");

    thread::sleep(Duration::from_secs(1));
    let new_temp = device.get_temp().await.unwrap();
    println!("new_temp = {}", new_temp);
    assert_ne!(new_temp, temp, "temp should change");
}
