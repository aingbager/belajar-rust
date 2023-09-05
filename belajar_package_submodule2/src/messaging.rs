const SOME_MESAGE: &str = "hello world";

pub mod service_layer {
    pub fn some_black_magic() {
        println!("{}", crate::messaging::SOME_MESAGE)
    }

    pub fn sayhello() {
        some_black_magic();
    }
}
