use crate::ball::{Ball, BallType};
use crate::R;
use ndarray::array;
use std::iter::Filter;
use std::slice::IterMut;
// need to have a pool balls struct with methods to expose various subsets of balls of interest

pub struct PoolBalls {
    pub balls: Vec<Ball>,
}

impl PoolBalls {
    pub fn new() -> Self {
        PoolBalls { balls: rack() }
    }

    pub fn num_balls(&self) -> usize {
        self.balls.len()
    }

    pub fn cue_ball(&mut self) -> &mut Ball {
        &mut self.balls[16]
    }

    pub fn stripes(&mut self) -> impl Iterator<Item = &mut Ball> {
        self.balls
            .iter_mut()
            .filter(|b| b.btype == BallType::STRIPED)
    }

    pub fn solids(&mut self) -> impl Iterator<Item = &mut Ball> {
        self.balls.iter_mut().filter(|b| b.btype == BallType::SOLID)
    }
}

fn assign_btype(n: usize) -> BallType {
    match n {
        1..=7 => BallType::SOLID,
        8 => BallType::EIGHT,
        9..=15 => BallType::STRIPED,
        _ => BallType::CUE,
    }
}

fn apply_triangle(balls: &mut Vec<Ball>) {
    let mut index = 0;
    for row in (1..=5).rev() {
        let y = R * (1.0 + (row as f64) * (3.0 as f64).sqrt());
        for j in 0..row {
            let x = R * ((row as f64) + (2 * j) as f64);
            balls[index].r = array![x, y];
            index += 1;
        }
    }
}

fn sort_balls(balls: &mut Vec<Ball>) {
    // moving 1ball to head of triangle
    let r_tmp = balls[0].r.clone();
    balls[0].r = balls[14].r.clone();
    balls[14].r = r_tmp;
    balls[0].btype = BallType::STRIPED;
    balls[14].btype = BallType::SOLID;
    balls.swap(7, 10);
}

fn rack() -> Vec<Ball> {
    let mut balls = (1..17)
        .map(|n| {
            let btype = assign_btype(n);
            Ball::new(n, btype)
        })
        .collect();

    apply_triangle(&mut balls);
    sort_balls(&mut balls);
    balls
}
