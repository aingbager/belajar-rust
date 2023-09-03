use rand::Rng;

pub:q fn acak_angka() -> i32 {
    rand::thread_rng().gen_range(1..=9)
}
