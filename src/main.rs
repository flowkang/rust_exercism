mod assembly_line;

pub fn hello() -> &'static str {
    "Hello, World!"
}

pub fn main_for_hello_world() {
    println!("{}", hello());
}


pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    2 * number_of_layers
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    actual_minutes_in_oven + preparation_time_in_minutes(number_of_layers)
}

pub fn main_for_lucians_luscious_lasagna() {
    println!("{}", expected_minutes_in_oven());
    println!("{}", remaining_minutes_in_oven(30));
    println!("{}", preparation_time_in_minutes(2));
    println!("{}", elapsed_time_in_minutes(3, 20));
}

pub fn main_for_assembly_line() {
    println!("{}", assembly_line::production_rate_per_hour(6));
    println!("{}", assembly_line::working_items_per_minute(6));
}

fn main() {
    // main_for_hello_world();
    // main_for_lucians_luscious_lasagna();
    main_for_assembly_line();
}
