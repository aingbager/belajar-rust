fn main() {
    //loop dengan label
    //
    let mut x = 1;

    // 'NAMALABEL: loop {
    //
    // }

    'ANJAY: loop {
        println!("x ke: {}", x);
        x += 1;

        if x == 500 {
            break 'ANJAY;
        }
    }
}
