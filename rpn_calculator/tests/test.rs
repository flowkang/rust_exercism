use rpn_calculator::CalculatorInput::{Add, Divide, Multiply, Subtract, Value};
use rpn_calculator::evaluate;

#[test]
fn test_1() {
    assert_eq!(evaluate(&[Value(4)]), Some(4));
    assert_eq!(evaluate(&[Value(4), Value(8), Add]), Some(12));
    assert_eq!(evaluate(&[Value(4), Value(8), Add, Value(7), Value(5), Subtract, Divide]), Some(6));
}

#[test]
fn test_2() {
    let input = [Add, Value(2), Value(2), Multiply];
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_3() {
    let input = [Value(2), Add];
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_4() {
    let input = [Value(2), Value(2)];
    assert_eq!(evaluate(&input), None);
}
