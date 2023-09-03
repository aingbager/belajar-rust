#[derive(Debug)]
struct LegoSet {
    code: i32,
    name: String,
    category: String,
}

impl LegoSet {
    fn new(code: i32, name: String, category: String) -> LegoSet {
        LegoSet {
            code,
            name,
            category,
        }
    }
}

fn main() {
    let rough_terrain_crane = LegoSet {
        code: 42082,
        name: String::from("Rough Terrain Crane"),
        category: String::from("Technic"),
    };
    println!("{:#?}", rough_terrain_crane);

    let xtreme_offroader = LegoSet::new(
        42099,
        String::from("4X4 X-treme Off-Roader"),
        String::from("Technic"),
    );
    println!("{:#?}", xtreme_offroader);
}
