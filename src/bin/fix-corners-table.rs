use std::fs::File;
use std::path::Path;

use flate2::read::GzDecoder;

fn main() {
    let path = Path::new("tables/corners.data.gz");
    let file = File::open(&path).unwrap();

    let d = GzDecoder::new(file);

    let solution_table: Vec<u8> = bincode::deserialize_from(d).unwrap();
    solution_table[0] = 0;

    let path = Path::new("tables/corners.data");
    let file = File::open(&path).unwrap();

    bincode::serialize_to(&file);
}
