use std::thread;
use std::time::Duration;

fn main() {
    //belajar while

    let mut num = 6;

    let mut h = 0;

    while num == 6 {
        h += 1;
        println!("hello wprld ke {}", h);
        thread::sleep(Duration::from_secs(1));

        if h == 60 {
            num = 7;
            break;
        }
    }

    println!("{}", num);
}
