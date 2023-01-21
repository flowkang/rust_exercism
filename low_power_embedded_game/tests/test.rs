use low_power_embedded_game::{divmod, evens, Position};

#[test]
fn test_divmod() {
    assert_eq!(divmod(10, 3), (3, 1));
}

#[test]
fn test_enumerate() {
    let a = 0_u8..;
    let mut enumerated = a.enumerate();
    assert_eq!(enumerated.next(), Some((0, 0)));
    assert_eq!(enumerated.next(), Some((1, 1)));
    assert_eq!(enumerated.next(), Some((2, 2)));
    assert_eq!(enumerated.next(), Some((3, 3)));
}

#[test]
fn test_filter() {
    let iter = 0_u8..;
    let mut iter2 = iter.enumerate().filter(|(i, _)| i % 2 == 0);
    assert_eq!(iter2.next(), Some((0, 0)));
    assert_eq!(iter2.next(), Some((2, 2)));
    assert_eq!(iter2.next(), Some((4, 4)));
}

#[test]
fn test_evens() {
    let mut even_ints = evens(0_u8..);
    assert_eq!(even_ints.next(), Some(0));
    assert_eq!(even_ints.next(), Some(2));
    assert_eq!(even_ints.next(), Some(4));
    assert_eq!(even_ints.next(), Some(6));

    let mut evens_from_odds = evens(1_i16..);
    assert_eq!(evens_from_odds.next(), Some(1));
    assert_eq!(evens_from_odds.next(), Some(3));
    assert_eq!(evens_from_odds.next(), Some(5));
    assert_eq!(evens_from_odds.next(), Some(7));
}

#[test]
fn test_manhattan() {
    assert_eq!(Position(3, 4).manhattan(), 7);
}
