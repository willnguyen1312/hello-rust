fn main() {
    let vec = vec![1, 2, 3];
    let value = vec.get(3).unwrap_or(&0);

    println!("Value: {:?}", value);
}
