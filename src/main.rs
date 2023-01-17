pub fn hello() -> &'static str {
    "Hello, World!"
}

pub fn main_for_hello_world() {
    println!("{}", hello());
}


fn main() {
    main_for_hello_world();
}
