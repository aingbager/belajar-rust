fn main() {
    let helo = hello();
    println!("{:?}", helo);
}

fn hello() -> String {
    let mut x = String::from("hello");
    {
        let y = String::from("hello world");
        x = y;
    }
}
