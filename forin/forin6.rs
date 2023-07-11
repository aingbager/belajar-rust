fn main() {
    for data in 0..=5 {
        println!("{}", data);
    }

    'nama: for i in 0..5 {
        if i == 3 {
            break 'nama;
        }
        println!("{i}");
    }
}
