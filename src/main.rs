use cube_solver::search;

fn main() {
    let sol = search::solve_exact();
    println!("{}", sol.len());
}
