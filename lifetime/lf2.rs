fn main() {
    //belajar lifetime
    let name = String::from("selamat datang para tamu undangan");
    hello(&name);

}

fn hello(h: &String){
    println!("bos: {}",h);
}
