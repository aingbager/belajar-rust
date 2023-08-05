fn main() {
    //belajar tuple
    let name = "udin";
    let name2 = "otong";
    let number = 5;
    let benar = true;

    let tuple_a = (name, name2, number, benar);

    println!("name1 = {:?}", tuple_a.0);
    println!("name2 = {:?}", tuple_a.1);
    println!("number = {:?}", tuple_a.2);
    println!("benar = {:?}", tuple_a.3);
}
