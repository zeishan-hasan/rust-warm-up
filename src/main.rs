fn calculator(val_1: i32, val_2: i32, opr: String) -> (i32, String) {
    match opr.as_str() {
        "+" => (val_1 + val_2, String::from("+")),
        "-" => (val_1 - val_2, String::from("-")),
        "*" => (val_1 * val_2, String::from("*")),
        "/" => {
            if val_2 == 0 {
                return (0, String::from("Cannot divide by zero!"));
            }
        (val_1 / val_2, String::from("/"))
        },
        _ => (0, String::from("Invalid operator")),
    }
}

fn main() {
    let x = 2;
    let y = 0;
    let operator = String::from("l");
    let result = calculator(x, y, operator);
    println!("{} {} {} = {}", x, result.1, y, result.0);
}
