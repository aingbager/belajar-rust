enum Buahbuahan {
    Apel,
    Nanas,
    Anggur,
}

fn main() {
    let hoby: Buahbuahan = Buahbuahan::Anggur;

    match hoby {
        Buahbuahan::Apel => {
            println!("apel yang enak");
        }
        Buahbuahan::Nanas => {
            println!("nanas yang asam");
        }
        Buahbuahan::Anggur => {
            println!("anggur yang paling enak");
        }
    }
}
