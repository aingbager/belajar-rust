fn main() {
    //borrowing
    let x = String::from("helo");
    let y = &x;

    println!("{}", x);
    println!("{}", y);

    //borrowing &mut

    let mut b = &mut a;
    *b = String::from("dunia"); //tambahkan * untuk mengambil nilainya

    println!("{}", a);
    println!("{:?}", b);
}
