fn main() {
    //deklarasi multy variable
    let (angka, string1) = (25, "uding");

    println!("{0}", angka);
    println!("{0}", string1);

    //atau
    let (nama, angka1): (&str, i32) = ("ucup", 44);
    println!("{0}", nama);
    println!("{0}", angka1);
}
