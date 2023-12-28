use device_query::{DeviceQuery, DeviceState};

pub fn get_mouse_position() -> (i32, i32) {
    let device_state = DeviceState::new();
    let mouse_pos = device_state.get_mouse().coords;
    mouse_pos
}
