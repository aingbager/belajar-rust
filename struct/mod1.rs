#[derive(Debug)]
pub struct BuahBuahan {
    apel: String,
    melon: String,
    harga: u32,
}

impl BuahBuahan {
    pub fn new(apel: String, melon: String, harga: u32) -> BuahBuahan {
        BuahBuahan { apel, melon, harga }
    }
}
