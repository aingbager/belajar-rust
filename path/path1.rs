
fn main() {
    // belajar path
    //

    //variable penampung inputan user
    let mut pesan = std::string::String::new();

    println!("namamu: ");

    // object baca inputan user
    let input_masukan = std::io::stdin();

    let hasil = input_masukan.read_line(&mut pesan);

    if hasil.is_err(){
        println!("inputan eror bro {:?}",hasil.err());
    }

    println!("inputan anda: {}",pesan);


}
