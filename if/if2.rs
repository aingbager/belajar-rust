fn main() {
    let angka1 = 5;

    let result: &str = if angka1 == 5 {
        "ini angka 5"
    } else if angka1 == 7 {
        "ini angka 7"
    } else {
        "bukan angka yang dicari"
    };

    println!("rsultnya : {}",result);
}
