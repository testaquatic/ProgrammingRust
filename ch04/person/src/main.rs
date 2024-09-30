struct Person {
    name: Option<String>,
    birth: i32,
}

fn main() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });
    // let first_name = composers[0].name;
}
