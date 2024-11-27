fn main() {
    struct Person {
        name: Option<String>,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
    });

    let first_name = composers[0].name.take();
    println!("{:?}", first_name);

    println!("{:?}", composers[0].name);
}
