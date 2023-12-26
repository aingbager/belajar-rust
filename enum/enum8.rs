#[derive(Debug)]
enum Angka {
    Angka1,
    Angka2,
    Angka3,
}
fn main() {
    //
    let favorite = Angka::Angka2;
    match favorite {
        Angka::Angka1 => {
            println!("no 1");
        }
        Angka::Angka2 => {
        println!("no 2");
        }
        Angka::Angka3 => {
            println!("no 3");
        }
    }
}
