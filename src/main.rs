fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let int_list = vec![34, 50, 25, 100, 65];
    let float_list = vec![34.2, 50.23, 25.25, 100.96, 65.88];
    let result = largest(&int_list);
    println!("The largest element is {result}");
    let result = largest(&float_list);
    println!("The largest element is {result}");
}
