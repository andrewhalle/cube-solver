use crate::cube::Cube;
use std::collections::VecDeque;

struct SearchNode<'a> {
    state: Cube<'a>,
    moves: String,
}

impl<'a> SearchNode<'a> {
    fn neighbors(&self) -> VecDeque<SearchNode<'a>> {
        let mut result = VecDeque::new();
        let all_moves = vec![
            "U", "U'", "U2", "D", "D'", "D2", "F", "F'", "F2", "B", "B'", "B2", "L", "L'", "L2",
            "R", "R'", "R2",
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

pub fn bfs(start: Cube) -> Option<String> {
    let mut queue = VecDeque::new();
    queue.push_back(SearchNode {
        state: start,
        moves: String::new(),
    });
    while let Some(curr) = queue.pop_front() {
        if curr.state.is_solved() {
            return Some(curr.moves);
        }

        queue.append(&mut curr.neighbors());
    }

    None
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
