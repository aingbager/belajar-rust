fn tambah(angka1: i32, angka2 :i32) -> i32{
    angka1 + angka2
}

fn helloworld(kata1 :&str , kata2 :&str){
    println!("hy {kata1}, lu {kata2}");
}

fn main() {
    //tanpa return
    let hasil = tambah(2,5);
    println!("{hasil}");

    helloworld("asu","kau");
}
