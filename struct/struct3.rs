struct Name {
    name: String,
    age: u8,
    alamat: String,
}

fn main() {
    let cotong = Name {
        name: String::from("cotong"),
        age: 25,
        alamat: String::from("kp ciantra"),
    };

    println!("namamu: {}", cotong.name);
    println!("usiamu: {}", cotong.age);
    println!("alamatmu: {}", cotong.alamat);
}
