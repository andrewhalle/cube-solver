use std::fs::File;
use std::path::Path;

use cube_solver::cube;
use cube_solver::cube::Cube;
use cube_solver::search;
use cube_solver::transformations;

fn main() {
    let path = Path::new("tables/edges1.data");
    let file = File::create(&path).unwrap();

    let t = transformations::cube3();
    let c = Cube::edges1(&t);
    let sol = search::solve_exact(c, cube::corners_state);
    bincode::serialize_into(file, &sol).unwrap();
}
