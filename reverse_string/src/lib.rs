pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(reverse("hello"), "olleh");
    }

    #[test]
    fn test_1() {
        println!("{:?}", "hello".chars());
        for e in "hello".chars() {
            println!("{}", e);
        }
        for (idx, e) in "hello".chars().enumerate() {
            println!("{} {}", idx, e);
        }
        println!("{:?}", "hello".chars().rev());
        for e in "hello".chars().rev() {
            println!("{:?}", e);
        }
        let forward: String = "hello".chars().collect();
        let backward: String = "hello".chars().rev().collect();
        assert_eq!("hello".to_string(), forward);
        assert_eq!("olleh".to_string(), backward);

        let forward: Vec<_> = "hello".chars().collect();
        let backward: Vec<_> = "hello".chars().rev().collect();
        assert_eq!(vec!['h', 'e', 'l', 'l', 'o'], forward);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], backward);
    }
}
