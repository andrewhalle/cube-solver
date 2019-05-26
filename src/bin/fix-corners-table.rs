use std::fs::File;
use std::path::Path;

use flate2::read::GzDecoder;

fn main() {
    let path = Path::new("tables/corners.data.gz");
    let file = File::open(&path).unwrap();

    let d = GzDecoder::new(file);

    let mut solution_table: Vec<u8> = bincode::deserialize_from(d).unwrap();
    solution_table[0] = 0;

    dbg!();
    let path = Path::new("tables/corners.data");
    let file = File::create(&path).unwrap();

    bincode::serialize_into(file, &solution_table).unwrap();
}
