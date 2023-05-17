use std::thread::sleep;
use std::time::Duration;

fn main() {
    //delay
    //
    let mut x = 1;

    while x < 10 {
        println!("x ke: {x}");
        x += 1;

        //delay
        sleep(Duration::from_secs(1));
    }

    println!("program selesai");
}
