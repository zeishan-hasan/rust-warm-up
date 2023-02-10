use std::io;

fn calculator(val_1: i32, val_2: i32, opr: &str) -> Result<i32, &str> {
    match opr {
        "+" => Ok(val_1 + val_2),
        "-" => Ok(val_1 - val_2),
        "*" => Ok(val_1 * val_2),
        "/" => {
            if val_2 == 0 {
                return Err("Cannot divide by zero!");
            }
        Ok(val_1 / val_2)
        },
        _ => Err("Invalid operator"),
    }
}

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_op = String::new();

    eprint!("Enter the first value: ");
    io::stdin()
        .read_line(&mut input_1)
        .expect("Failed to read the first value.");
    let input_1: i32 = input_1.trim().parse().expect("Please enter a number!");
    
    eprint!("Enter the second value: ");
    io::stdin()
        .read_line(&mut input_2)
        .expect("Failed to read the second value.");
    let input_2: i32 = input_2.trim().parse().expect("Please enter a number!");

    eprint!("Enter the operator: ");
    io::stdin()
        .read_line(&mut input_op)
        .expect("Failed to read the operator");
    let input_op = input_op.trim();
    
    let result = calculator(input_1, input_2, input_op);
    if result.is_ok() {
    println!("Result:\n{} {} {} = {}", input_1, input_op, input_2, result.unwrap());
    } else {
        println!("{:#?}", result.err().unwrap());
    }
}
