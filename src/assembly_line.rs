pub fn production_rate_per_hour(speed: u8) -> f64 {
    let default_rate_per_hour = speed as f64 * 221.0;
    let rate_list = [1.0, 0.9, 0.77];
    let speed_step = speed / 4;
    default_rate_per_hour * rate_list[speed_step as usize]
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
