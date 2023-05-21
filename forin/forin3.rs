fn main() {
    // for in
    'INILABEL: for angka in 1..6 {
        println!("{angka}");

        if angka > 3 {
            break 'INILABEL;
        }
    }
}
