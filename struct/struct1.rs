struct Name {
    name: String,
    age: u16,
    gender: bool,
}

fn main() {
    let user1 = Name {
        name: String::from("udin"),
        age: 20,
        gender: true,
    };

    println!("name: {}", user1.name);
    println!("age: {}", user1.age);
    println!("gender: {}", user1.gender);
}
