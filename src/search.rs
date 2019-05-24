use crate::cube::Cube;
use std::collections::{HashMap, HashSet, VecDeque};

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
    fn neighbors(&self) -> VecDeque<SearchNodeSmall<'a>> {
        let mut result = VecDeque::new();
        let all_moves = vec![
            "U", "U'", "U2", "F", "F'", "F2", "R", "R'", "R2", "D", "D'", "D2", "B", "B'", "B2",
            "L", "L'", "L2",
        ];

        for m in all_moves.iter() {
            let mut state = self.state.clone();
            state.twist(m);

            let distance = self.distance + 1;
            result.push_back(SearchNodeSmall { state, distance });
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
        for neighbor in neighbors.into_iter() {
            if solution_table[index_fn(&neighbor.state)] == 0 {
                solution_table[index_fn(&neighbor.state)] = curr.distance + 1;
                queue.push_back(neighbor);
                println!("{}", counter);
                counter += 1;
            }
        }
    }

    solution_table
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
}
