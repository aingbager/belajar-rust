struct Angka {
    property1: String,
    property2: u8,
    property3: Vec<String>,
    property4: bool,
}

fn main() {
    //pendefinisian variable haris di awal nama variable: namastruct
    //dan untuk pengisian struct = struct
    let angka1: Angka = Angka {
        property1: String::from("hello"),
        property2: 66,
        property3: vec![String::from("udin"), String::from("surudin")],
        property4: true,
    }; // pengisian struct harus di akhiri titik koma(;)

    //pemanggilan property(print)
    println!("property1 = {}", angka1.property1);
    println!("property2 = {}", angka1.property2);
    println!("property3 = {:?}", angka1.property3);
    println!("property4 = {}", angka1.property4);
}
