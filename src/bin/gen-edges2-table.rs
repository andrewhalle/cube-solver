use std::fs::File;
use std::path::Path;

use cube_solver::cube;
use cube_solver::cube::Cube;
use cube_solver::search;
use cube_solver::transformations;

fn main() {
    let path = Path::new("tables/edges2.data");
    let file = File::create(&path).unwrap();

    let t = transformations::cube3();
    let c = Cube::new(3, &t);

    let sol = search::gen_table(c, 42577920, cube::edges2_index);

    bincode::serialize_into(file, &sol).unwrap();
}
