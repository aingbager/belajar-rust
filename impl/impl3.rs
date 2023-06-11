/* #[derive(Debug)] */
struct NamaUser{
    nama1: String,
    nama2: String,
}

impl NamaUser {
    fn user(nama1: String, nama2: String) ->NamaUser{
        NamaUser{nama1,nama2}
    }
}
fn main() {
    let user1 = NamaUser{
        nama1: String::from("udin"),
        nama2: String::from("surudin"),
    };

    println!("{:?}",user1);

    
    
    let user2 = NamaUser::user(
        String::from("otong"),
        String::from("surotong"),
    );

    println!("{:?}",user2);
}
