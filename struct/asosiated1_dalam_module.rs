mod mod1;

fn main() {
    let buah = mod1::BuahBuahan::new(String::from("apel"), String::from("melon"), 2500);

    println!("{:#?}", buah);
}
