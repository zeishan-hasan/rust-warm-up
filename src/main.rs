fn main() {
    let number_1:f32 = 3.45;
    let number_2:f32 = takes_and_gives_back(number_1);

    // Since the float (and also all integer, boolean, characte) type
    // has the copy trait the below line won't produce error as the ownership
    // is not transferred to takes_and_gives_back()rather shared with it.
    println!("{number_1}");

    println!("{number_2}");
}

fn takes_and_gives_back(number: f32) -> f32 {
    number
}
