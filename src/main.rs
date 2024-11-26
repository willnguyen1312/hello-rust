fn main() {
    let vec = vec![1, 2, 3];

    match vec.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short."),
    }
}
