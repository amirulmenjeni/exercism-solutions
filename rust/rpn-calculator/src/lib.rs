#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    
    let mut rpn_stack = Vec::new();

    for e in inputs {
        match e {

            &CalculatorInput::Value(number) => rpn_stack.push(number),

            _ => {
                match rpn_stack.pop() {
                    Some(rhs) => {
                        if let Some(lhs) = rpn_stack.pop() {
                            match e {
                                CalculatorInput::Add => rpn_stack.push(lhs + rhs),
                                CalculatorInput::Subtract => rpn_stack.push(lhs - rhs),
                                CalculatorInput::Multiply => rpn_stack.push(lhs * rhs),
                                CalculatorInput::Divide => rpn_stack.push(lhs / rhs),
                                _ => unreachable!(), // Won't be reachable.
                            }
                        }
                    },
                    
                    // Trying to operate without operands!
                    None => break,
                }
            }
        }
    }
    
    // The stack should always evaluate to a length-one vector.
    // If we ends up with more than one values in the stack, this means there's too many operands
    // for any of operators (+,-,*,/), i.e., more than two, the lhs and rhs operands.
    match rpn_stack.len() {
        1 => rpn_stack.pop(),
        _ => None,
    }
}
