fn main() {
    let some_value = 2;

    println!("Initial value: {}", some_value);

    some_value = addOne(some_value);

    println!("2 + 1 = ");
}

fn addOne(x: i32) -> i32 {
    x + 1;
}
