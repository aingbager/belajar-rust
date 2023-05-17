fn main() {
    let angka = 2;

    let result: &str = if angka == 2 {
        "ya ini angka 2"
    } else if angka > 2 {
        "ini lebih besar dari 2"
    } else {
        "ini lebih kecil"
    };

    println!("angka adalah {result}");
}
