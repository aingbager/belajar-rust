fn main() {
    let mut my_loop = 0;

    let result = loop {
        my_loop += 1;

        if my_loop == 10 {
            break my_loop * 2;
        }
    };
    println!("hasil result: {result}");
}
