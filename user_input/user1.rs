fn main() {
    //belajar user input

    println!("masukan namamu:");
    let mut input_user = std::string::String::new();

    //object reader
    let penampung = std::io::stdin();

    //proses input
    let hasil = penampung.read_line(&mut input_user);

    //pengecekan error
    if hasil.is_err() {
        println!("program error {:?}", hasil.err());
    }

    println!("hasil namamu: {}", input_user);
}
