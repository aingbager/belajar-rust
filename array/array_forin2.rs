fn main() {
    //array for in
    let name: [&str;5] = ["merah","kuning","hijau","biru","ungu"];

    for names in name{
        println!("{}",names);
    }

    let name2 = name.len();
    println!("size:{}",name2);
}
