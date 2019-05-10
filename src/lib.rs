use lazy_static::lazy_static;
use ndarray::prelude::*;

lazy_static! {
    // put transformation matricies here
}

// implement cube as column vector, turns are matrix transformations
pub struct Cube;

pub struct Move {
    face: Face,
    turn: Turn,
}

pub enum Face {
    U,
    D,
    F,
    B,
    L,
    R,
}

pub enum Turn {
    Clockwise,
    CounterClockwise,
    Double,
}

impl Cube {
    pub fn new() -> Cube {
        Cube
    }

    pub fn apply(&mut self, moves: Vec<Move>) {}

    pub fn scramble(&mut self) {}

    pub fn solve(&self) -> Vec<Move> {
        vec![]
    }
}
