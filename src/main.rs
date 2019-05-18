use std::fs::File;
use std::path::Path;

use cube_solver::search;

fn main() {
    let path = Path::new("tables/corners.json");
    let file = File::create(&path).unwrap();

    let sol = search::solve_exact();
    serde_json::to_writer(file, &sol).unwrap();
}
