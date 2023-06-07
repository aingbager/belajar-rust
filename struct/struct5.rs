struct Name {
    name: String,
    age: i32,
    kumpulan: Vec<String>,
    healt: bool,
}

fn main() {
    let group1 = Name {
        name: String::from("udin"),
        age: 25,
        kumpulan: vec![
            String::from("udin"),
            String::from("otong"),
            String::from("dodon"),
        ],
        healt: true,
    };

    println!("name: {}", group1.name);
    println!("usia: {}", group1.age);
    println!("group name: {:?}", group1.kumpulan);
    println!("healt: {}", group1.healt);
}
