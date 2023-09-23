fn main() {
    //belajar borrowing
    let x = 5;
    let z = &x;

    println!("x: {}", x);
    println!("z: {}", z);

    let mut a = String::from("udin");
    let mut b = &mut a;
    *b = String::from("surudin");

    println!("{:?}", a);
    println!("{:?}", b);
}
