fn main() {
    //type data slice
    let name = ["udin", "idin", "odon"];

    let result = &name[0..=2];

    println!("{:?}", result);
}
