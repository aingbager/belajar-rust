fn main() {
    let mut x = 1;

    let zoloo = loop {
        x += 1;

        if x == 100 {
            break x * 2;
        }
    };
    println!("zoloo: {}", zoloo);
}
