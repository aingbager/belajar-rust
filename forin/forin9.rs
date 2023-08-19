fn main() {
    //belajar for in
    'perulangan: for i in 0..5{
        println!("{i}");
         if i > 3 {
            break 'perulangan;
        }
    }
}
