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
    let x = 2;
    let y = 0;
    let operator = "&";
    let result = calculator(x, y, operator);
    if result.is_ok() {
    println!("{} {} {} = {:?}", x, operator, y, result.unwrap());
    } else {
        println!("{:#?}", result.err().unwrap());
    }
}
