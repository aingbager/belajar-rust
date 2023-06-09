#[derive(Debug)]
struct NamaBuah {
    sayur: String,
    buah: String,
}

impl NamaBuah {
    fn user(sayur: String,buah: String)-> NamaBuah{
        NamaBuah{sayur,buah}
    }
}

fn main() {
    //belajar impl(implementasi)
    let user1 = NamaBuah{
        sayur: String::from("kangkung"),
        buah: String::from("apel"),
    };

    // print struct
    println!("type struct sayur: {:?}",user1.sayur);
    println!("type struct buah: {:?}",user1.buah);

    //print impl
    let user2 = NamaBuah::user(
        String::from("bayam"),
        String::from("jeruk"),
    );
    
    println!("type impl: {:?}",user2);

}
