enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operator: Operator, num1: i32, num2: i32) -> Option<i32> {
    match operator {
        Operator::Add => Some(num1 + num2),
        Operator::Subtract => Some(num1 - num2),
        Operator::Multiply => Some(num1 * num2),
        Operator::Divide => {
            if num2 != 0 {
                Some(num1 / num2)
            } else {
                None
            }
        }
    }
}

fn main() {
    let operator = Operator::Divide;  // Using the Divide operator
    let num1 = 10;
    let num2 = 5;

    match calculate(operator, num1, num2) {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: Division by zero"),
    }
}
