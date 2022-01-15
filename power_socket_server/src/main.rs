use power_socket_server::{serve, State};

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let state = State::new();
    serve(state, 1234).unwrap();
}
