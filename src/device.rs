use device_query::{DeviceQuery, DeviceState};

pub fn get_mouse_coords() -> (i32, i32) {
    DeviceState::new().get_mouse().coords
}
