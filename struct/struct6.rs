#![warn(dead_code)]
struct Susu {
    name: String
}

fn main() {
    let name: Susu = Susu{
        name: String::from("otong"),
    };

    println!("name: {:?}",name);
}
