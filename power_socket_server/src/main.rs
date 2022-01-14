use power_socket_server::{serve, State};

fn main() {
    let state = State::new();
    serve(state, 1234).unwrap();
}
