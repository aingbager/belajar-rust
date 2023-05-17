fn main() {
    // label dalam for in
    'LABEL_NUMBER: for angka in 0..=10 {
        if angka > 3 {
            break 'LABEL_NUMBER;
        }
        println!("{angka}");
    }
}
