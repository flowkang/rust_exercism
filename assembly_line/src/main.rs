mod assembly_line;

fn main() {
    println!("{}", assembly_line::production_rate_per_hour(6));
    println!("{}", assembly_line::working_items_per_minute(6));
}
