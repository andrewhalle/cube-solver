use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::path::Path;

use flate2::read::GzDecoder;
use lazy_static::lazy_static;

use crate::cube;
use crate::cube::Cube;

lazy_static! {
    static ref CORNERS_TABLE: Vec<u8> = load_corners_table();
}

fn load_corners_table() -> Vec<u8> {
    let path = Path::new("tables/corners.data.gz");
    let file = File::open(&path).unwrap();
    let d = GzDecoder::new(file);

    bincode::deserialize_from(d).unwrap()
}

struct SearchNode<'a> {
    state: Cube<'a>,
    moves: String,
}

struct SearchNodeSmall<'a> {
    state: Cube<'a>,
    distance: u8,
}

impl<'a> SearchNode<'a> {
    fn neighbors(&self) -> VecDeque<SearchNode<'a>> {
        let mut result = VecDeque::new();
        let all_moves = vec![
            "U", "U'", "U2", "F", "F'", "F2", "R", "R'", "R2", "D", "D'", "D2", "B", "B'", "B2",
            "L", "L'", "L2",
        ];

        for m in all_moves.iter() {
            let mut state = self.state.clone();
            state.twist(m);

            let moves = format!("{} {}", self.moves, m);
            result.push_back(SearchNode { state, moves });
        }

        result
    }
}

impl<'a> SearchNodeSmall<'a> {
    fn neighbors(&self) -> VecDeque<(SearchNodeSmall<'a>, String)> {
        let mut result = VecDeque::new();
        let all_moves = vec![
            "U", "U'", "U2", "F", "F'", "F2", "R", "R'", "R2", "D", "D'", "D2", "B", "B'", "B2",
            "L", "L'", "L2",
        ];

        for m in all_moves.into_iter() {
            let mut state = self.state.clone();
            state.twist(m);

            let distance = self.distance + 1;
            result.push_back((SearchNodeSmall { state, distance }, m.to_string()));
        }

        result
    }
}

pub fn bfs(start: Cube) -> Option<String> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(SearchNode {
        state: start,
        moves: String::new(),
    });
    while let Some(curr) = queue.pop_front() {
        if curr.state.is_solved() {
            return Some(curr.moves);
        }

        let mut neighbors = curr.neighbors();
        while let Some(neighbor) = neighbors.pop_front() {
            let state_string = neighbor.state.state_string();
            if !seen.contains(&state_string) {
                queue.push_back(neighbor);
                seen.insert(state_string);
            }
        }
    }

    None
}

pub fn solve_exact<F: Fn(&Cube) -> String>(c: Cube, state_string: F) -> HashMap<String, String> {
    let mut queue = VecDeque::new();
    let mut solution_table = HashMap::new();
    queue.push_back(SearchNode {
        state: c,
        moves: String::new(),
    });
    while let Some(curr) = queue.pop_front() {
        let neighbors = curr.neighbors();
        for neighbor in neighbors.into_iter() {
            let state_string = state_string(&neighbor.state);
            if !solution_table.contains_key(&state_string) {
                solution_table.insert(state_string, neighbor.moves.clone());
                queue.push_back(neighbor);
                println!("{}", solution_table.len());
            }
        }
    }

    solution_table
}

pub fn gen_table<F: Fn(&Cube) -> usize>(c: Cube, result_size: usize, index_fn: F) -> Vec<u8> {
    let mut solution_table = vec![0 as u8; result_size];
    let mut queue = VecDeque::new();
    let mut counter = 0 as usize;

    queue.push_back(SearchNodeSmall {
        state: c,
        distance: 0,
    });

    while let Some(curr) = queue.pop_front() {
        let neighbors = curr.neighbors();
        for (neighbor, _) in neighbors.into_iter() {
            if solution_table[index_fn(&neighbor.state)] == 0 {
                solution_table[index_fn(&neighbor.state)] = curr.distance + 1;
                queue.push_back(neighbor);
                println!(
                    "states visited: {}, distance from solved: {}",
                    counter, curr.distance
                );
                counter += 1;
            }
        }
    }

    solution_table[0] = 0;

    solution_table
}

pub fn solve_corners(c: &Cube) -> String {
    let mut curr = SearchNodeSmall {
        state: c.clone(),
        distance: CORNERS_TABLE[cube::corners_index(c)],
    };
    dbg!();

    let mut sol = String::new();
    while CORNERS_TABLE[cube::corners_index(&curr.state)] != 0 {
        let neighbors = curr.neighbors();
        let mut min_distance = 100;
        let mut next_move = String::new();

        for (neighbor, neighbor_move) in neighbors.into_iter() {
            let neighbor_distance = CORNERS_TABLE[cube::corners_index(&neighbor.state)];
            if neighbor_distance < min_distance {
                min_distance = neighbor_distance;
                next_move = neighbor_move;
                curr = neighbor;
            }
        }

        sol.push_str(&next_move);
        sol.push_str(" ");
        dbg!(&sol);
    }

    sol.trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::cube::Cube;
    use crate::search;
    use crate::transformations;

    #[test]
    fn it_works() {
        let t = transformations::cube2();
        let mut c = Cube::new(2, &t);
        c.twist("B' R D2 U2 R' L U D' F2 D' F2 L F2 L2 B");
        println!("{}", search::bfs(c).unwrap());
    }

    #[test]
    fn test_solve_corners() {
        let t = transformations::cube3();
        let mut c = Cube::new(3, &t);
        c.twist("B' R D2 U2 R' L U D' F2 D' F2 L F2 L2 B");
        println!("{}", search::solve_corners(&c));
    }
}
