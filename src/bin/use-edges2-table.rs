use std::fs::File;
use std::path::Path;

use flate2::read::GzDecoder;

fn main() {
    let path = Path::new("tables/edges2.data.gz");
    let file = File::open(&path).unwrap();

    let d = GzDecoder::new(file);

    let solution_table: Vec<u8> = bincode::deserialize_from(d).unwrap();

    let mut counter = 0;
    for i in solution_table.into_iter() {
        if i == 0 {
            counter += 1;
        }
    }

    println!("{}", counter);
}
