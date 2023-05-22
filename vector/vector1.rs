fn main() {
    //type data vector
    //
    let mut data = vec!["merah","kuning","hijau","biru"];

    data.remove(2);
    /* data.pop(); */
    println!("data: {:?}",data);
    println!("length: {}, capasity: {}",data.len() , data.capacity());
}
