pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = format!("{}", num);
    let degree = num_str.len() as u32;
    num_str.chars().into_iter().fold(
        0,
        |acc: u64, x: char| acc + x.to_digit(10).unwrap().pow(degree) as u64)
        == num as u64
}
