#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn get_calc_func(e: &CalculatorInput) -> fn(i32, i32) -> i32 {
    match e {
        CalculatorInput::Add => |a, b| a + b,
        CalculatorInput::Subtract => |a, b| a - b,
        CalculatorInput::Multiply => |a, b| a * b,
        CalculatorInput::Divide => |a, b| a / b,
        _ => panic!("not supported"),
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for e in inputs {
        match e {
            CalculatorInput::Value(value) => {
                stack.push(*value);
                continue;
            }
            _ => {
                let v2 = stack.pop();
                let v1 = stack.pop();
                if v2 == None || v1 == None {
                    return None;
                }

                let v1 = v1.unwrap();
                let v2 = v2.unwrap();
                stack.push(get_calc_func(e)(v1, v2));
            }
        }

    }

    if stack.len() == 1 { stack.pop() } else { None }
}

