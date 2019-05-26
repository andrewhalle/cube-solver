use std::fs::File;
use std::path::Path;

use cube_solver::cube;
use cube_solver::cube::Cube;
use cube_solver::search;
use cube_solver::transformations;

fn main() {
    let path = Path::new("tables/corners.data");
    let file = File::create(&path).unwrap();

    let t = transformations::cube3();
    let c = Cube::new(3, &t);

    // generate the corners heuristic table, have to reset solved
    // to 0, since it gets set to 2 during search, because detecting
    // if we've seen a node is if it's entry in the table is 0
    let sol = search::gen_table(c, 88179840, cube::corners_index);
    sol[0] = 0;

    bincode::serialize_into(file, &sol).unwrap();
}
