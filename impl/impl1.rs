#[derive(Debug)]
struct NamaMobil {
    honda: String,
    tahun: i32,
}

impl NamaMobil{
    fn new(honda: String,tahun: i32)->NamaMobil{
        NamaMobil{honda,tahun}
    }
}

fn main() {
    let user1 = NamaMobil{
        honda: String::from("zess"),
        tahun: 2018,
    };
    println!("{:#?}",user1);

    let user2 = NamaMobil::new(
        String::from("brio"),
        2017,
    );
    println!("{:#?}",user2);
}
