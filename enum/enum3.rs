enum Food {
  PenyetanTerangBulan,
  PizzaNanas,
}

fn main() {
let makanan_favorit: Food = Food::PenyetanTerangBulan;
match makanan_favorit {
      Food::PenyetanTerangBulan => {
          println!("your food taste is quite ... unique");
},
Food::PizzaNanas => {
          println!("it's morally wrong to have pineaple on top
of pizza");
}
