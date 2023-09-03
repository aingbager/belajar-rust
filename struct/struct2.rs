//variabel struct dengan pendevinisan tanpa fredivined value
struct Cars {
    brand: String,
    release: String,
}

fn main() {
    let mut car_three: Cars;

    car_three = Cars {
        brand: String::from("toyota"),
        release: String::from("2019"),
    };

    println!("mobil: {},tahun: {}", car_three.brand, car_three.release);

    //ubah nilainya
    car_three.brand = String::from("honda");
    car_three.release = String::from("jazz");

    println!("mobil: {},tahun: {}", car_three.brand, car_three.release);
}
