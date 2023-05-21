fn main() {
    let mut angka = 1;

    loop {
        angka += 1;
        println!("angka ke: {}", angka);

        // 500 ribu bisa di ketik dengan underscore(_)
        if angka == 5_00_000 {
            break;
        }
    }
    println!("program berhenti");
}
