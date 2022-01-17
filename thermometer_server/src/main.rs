use thermometer_server::serve;

fn main() {
    serve("127.0.0.1:12345").unwrap();
}
