fn main() {
    let mut angka = 0;
    let max = 20;

    loop {
        angka += 1;
        if angka % 2 == 1 {
            continue;
        }
        println!("angka ke: {angka}");

        if angka > max {
            break;
        }
    }
}
