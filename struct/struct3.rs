fn main() {
    struct Point {
        x: f32,
        y: f32,
    }
    let nilai = Point { x: 32.2, y: 12.4 };

    let Point { x: x_x, y: y_y } = nilai;

    println!("x = {}", x_x);
    println!("y = {}", y_y);
}
