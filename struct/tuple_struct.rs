#[derive(Debug)]
struct Color(r: i32,g: i32,b:i32);

impl Color {
    fn red()->self{
        self(255,0,0)
    }
    fn green()->self{
        self(0,255,0)
    }
    fn blue()->self{
        self(0,0,255)
    }
}
fn main() {
    println!("{:#?}",Color::red());
}
