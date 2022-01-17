use smart_home::devices::thermometer::Thermometer;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_with_mock_server() {
    let arc = Thermometer::new("test power socket", "", "127.0.0.1:1234");
    let arc_clone = Arc::clone(&arc);

    thread::spawn(|| thermometer_server::serve("127.0.0.1:1234").unwrap());
    thread::spawn(move || Thermometer::start_poll(arc_clone));

    println!("threads started, waiting");
    thread::sleep(Duration::from_secs(1));

    let guard = arc.read().unwrap();
    let temp = guard.get_temp();
    drop(guard);
    println!("temp = {}", temp);
    assert_ne!(temp, 20.0, "temp should not be default");

    thread::sleep(Duration::from_secs(1));
    let new_temp = arc.read().unwrap().get_temp();
    println!("new_temp = {}", new_temp);
    assert_ne!(new_temp, temp, "temp should change");
}
