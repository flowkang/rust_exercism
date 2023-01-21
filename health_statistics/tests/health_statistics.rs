use health_statistics::User;

#[test]
fn test_1() {

    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!("Bob", bob.name());
    assert_eq!(32, bob.age());
    assert_eq!(155.2, bob.weight());
    bob.set_age(33);
    assert_eq!(33, bob.age());
}