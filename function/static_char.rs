use std::io;
use std::io::Write;

fn main() {
    //belajar static function
    let mut input_user = String::new();
    print!("masukan age: ");
    io::stdout().flush().except("ini error");
    io::stdin().read_line(&mut input_user).expect("error");
    let age = 15;
    println!("age: {}", name(age));
}

fn name(age: i32) -> &'static str {
    if age <= 12 {
        return "anak anak";
    } else if age <= 27 {
        return "remaja";
    } else {
        return "dewasa";
    }
}
