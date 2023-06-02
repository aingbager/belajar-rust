struct Kata {
    kata: String,
    ucapan: String,
}
fn main() {
    let nama = Kata {
        kata: String::from("katanya"),
        ucapan: String::from("ucapanmu"),
    };

    println!("kata: {}", nama.kata);
    println!("ucapan: {}", nama.ucapan);
}
