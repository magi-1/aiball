use crate::ball::{Ball, BallType};
use ndarray::{array, Array1};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn assign_btype(n: usize) -> BallType {
    match n {
        1..=7 => BallType::SOLID,
        8 => BallType::EIGHT,
        9..=15 => BallType::STRIPED,
        _ => BallType::CUE,
    }
}

fn assign_location(n: usize) -> Array1<f64> {
    array![0.0, 0.0]
}

pub fn rack() -> Vec<Ball> {
    // Initializing balls and shuffling
    //let mut numbers: Vec<usize> = (1..17).collect();
    //numbers.shuffle(&mut thread_rng(););

    let balls = (1..17)
        .map(|n| {
            let btype = assign_btype(n);
            let r = assign_location(n);
            Ball::new(btype, r)
        })
        .collect();
    balls
}
