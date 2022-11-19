use traffic_light::{InitializedGlobalDevice, State, Color};

fn main() {
    let device = InitializedGlobalDevice::create_with_any().unwrap();
    device.set_light(Color::Green, State::Off).unwrap();
}