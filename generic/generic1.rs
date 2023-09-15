fn nama<T: std::fmt::Debug>(a: i32, b: T) {
    println!("{}{:?}", a, b);
}
fn main() {
    //belajar generic
    nama(55, true);
}
