fn main() {
    let string_1 = gives_ownership();
    let string_2 = String::from("Welcome");
    let stirng_3 = takes_and_gives_back(string_2);

    println!("{}", string_1);

    // This will cause error since string_2 is moved to 'takes_and_gives_back()'
    // and string type does not have a copy trait.
    // println!("{}", string_2);
    
    println!("{}", stirng_3);

}

fn gives_ownership() -> String {
    let some_string = String::from("This is from gives_ownership().");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
