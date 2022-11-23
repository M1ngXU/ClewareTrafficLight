use cleware_traffic_light::{Color, InitializedGlobalDevice, State};

fn main() {
    InitializedGlobalDevice::set_light(Color::Green, State::Off).unwrap();
}
