use std::fs::File;
use std::path::Path;

use cube_solver::search;

fn main() {
    let path = Path::new("tables/corners.data");
    let file = File::create(&path).unwrap();

    let sol = search::solve_exact();
    bincode::serialize_into(file, &sol).unwrap();
}
