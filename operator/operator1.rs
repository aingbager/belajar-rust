fn main() {
    //operator perbandingan
    let angka1 = 12;
    let angka2 = 24;

    let hasil = angka1 == angka2; //false

    println!("{hasil}");

    let hasil2 = angka1 != angka2; //true
    println!("{hasil2}");

    //operator negasi !(not)

    let (x, y) = (12, -12);

    let a1 = -x == y;
    let a2 = !(x == y);

    println!("negasi1: {a1}");
    println!("negasi2: {a2}");
}
