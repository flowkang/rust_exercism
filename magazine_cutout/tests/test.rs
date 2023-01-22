use std::collections::HashMap;
use magazine_cutout::can_construct_note;

#[test]
fn it_works() {
    let magazine = "two times three is not four".split_whitespace().collect::<Vec<&str>>();
    let note = "two times two is four".split_whitespace().collect::<Vec<&str>>();
    assert!(!can_construct_note(&magazine, &note));

    let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'".split_whitespace().collect::<Vec<&str>>();
    let note = "Amy Mainzer chatting with Leonardo DiCaprio"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(can_construct_note(&magazine, &note));
}

#[test]
fn test_1() {
    let mut magazine: HashMap<&str, bool> = HashMap::new();
    "two times three is not four".split_whitespace().for_each(|x| {
        magazine.insert(x, true);
    });
    println!("{:?}", magazine);

    let result = magazine.remove("two");
    println!("{:?} {:?}", magazine, result);
    let result = magazine.remove("two");
    println!("{:?} {:?}", magazine, result);
}

#[test]
fn test_2() {
    let magazine = ["Enough", "is", "enough", "when", "enough", "is", "enough"];
    let note = ["enough", "is", "enough"];
    assert!(can_construct_note(&magazine, &note));
}
