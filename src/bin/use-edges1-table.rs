use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use flate2::read::GzDecoder;

use cube_solver::cube;
use cube_solver::cube::Cube;
use cube_solver::transformations;

fn main() {
    let path = Path::new("tables/edges1.data.gz");
    let file = File::open(&path).unwrap();

    let d = GzDecoder::new(file);

    let solution_table: HashMap<String, String> = bincode::deserialize_from(d).unwrap();

    let t = transformations::cube3();
    let mut c = Cube::edges1(&t);
    c.twist("B' R D2 U2 R' L U D' F2 D' F2 L F2 L2 B");

    println!("{}", solution_table.get(&cube::edges_state(&c)).unwrap());
}
