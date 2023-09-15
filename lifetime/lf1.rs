fn main() {
    let x;

    {
        let z = 5;
        x = &z;
    }

    println!("x: {}", x);
}
