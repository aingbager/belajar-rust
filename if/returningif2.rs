fn main() {
    //belajar returning if
    let x = 6;

    let nama: bool;
    if x == 5 {
        nama = false;
    }else {
        nama = true;
    }

    println!("boolean nama: {}",nama);

    if nama == true {
        println!("ini benar");
    }else {
        println!("ini salah");
    }
}
