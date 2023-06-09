fn main() {
    //returning loop
    let mut y = 0;

    let hasil = loop {
        y += 1;

        if y == 100{
            break y / 2
        }
    };

    println!("hasil = {}",hasil);
}
