use ambassador::{delegatable_trait, Delegate};
use ndarray::{Array1, Array2};
use roots::{find_roots_quartic, Roots};

use crate::ball::Ball;
use crate::{SimObjects, G, MU, R};

#[delegatable_trait]
pub trait Event {
    fn get_time_until(&self) -> f64;
    fn calculate_time_until(&mut self, objects: &SimObjects);
    fn apply(&self, balls: &mut Vec<Ball>);
}

#[derive(Delegate)]
#[delegate(Event)]
pub enum EventEnum {
    StopRolling(StopRolling),
    HitBall(HitBall),
    HitCushion(HitCushion),
    HitPocket(HitPocket),
    NullEvent(NullEvent),
}

impl Default for EventEnum {
    fn default() -> Self {
        EventEnum::NullEvent(NullEvent::new())
    }
}

impl EventEnum {
    pub fn mut_compare(&mut self, other: EventEnum) {
        if other.get_time_until() < self.get_time_until() {
            *self = other;
        }
    }
}

pub struct NullEvent {
    time_delta: f64,
}

impl NullEvent {
    pub fn new() -> Self {
        NullEvent {
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for NullEvent {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, _objects: &SimObjects) {}

    fn apply(&self, balls: &mut Vec<Ball>) {}
}

pub struct StopRolling {
    ball_id: usize,
    time_delta: f64,
}

impl StopRolling {
    pub fn new(ball_id: usize) -> Self {
        Self {
            ball_id,
            time_delta: 0.0,
        }
    }
}

impl Event for StopRolling {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, objects: &SimObjects) {
        let ball: &Ball = &objects.balls[self.ball_id];
        self.time_delta = ball.mag_v / (MU * G);
    }

    fn apply(&self, balls: &mut Vec<Ball>) {}
}

pub struct HitPocket {
    ball_id: usize,
    pocket_id: usize,
    time_delta: f64,
}

impl HitPocket {
    pub fn new(ball_id: usize, pocket_id: usize) -> Self {
        Self {
            ball_id,
            pocket_id,
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for HitPocket {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, _objects: &SimObjects) {}

    fn apply(&self, balls: &mut Vec<Ball>) {}
}

pub struct HitCushion {
    ball_id: usize,
    cushion_id: usize,
    time_delta: f64,
}

impl HitCushion {
    pub fn new(ball_id: usize, cushion_id: usize) -> Self {
        Self {
            ball_id,
            cushion_id,
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for HitCushion {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, _objects: &SimObjects) {}

    fn apply(&self, balls: &mut Vec<Ball>) {}
}

pub struct HitBall {
    ball_id: usize,
    other_ball_id: usize,
    time_delta: f64,
}

impl HitBall {
    pub fn new(ball_id: usize, other_ball_id: usize) -> Self {
        Self {
            ball_id,
            other_ball_id,
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for HitBall {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&mut self, objects: &SimObjects) {
        let b1: &Ball = &objects.balls[self.ball_id];
        let b2: &Ball = &objects.balls[self.other_ball_id];
        let c: Array2<f64> = &b1.r_coeffs - &b2.r_coeffs;

        let a4: f64 = c[[0, 2]].powi(2) + c[[1, 2]].powi(2);
        let a3: f64 = 2.0 * (c[[0, 2]] * c[[0, 1]] + c[[1, 2]] * c[[1, 1]]);
        let a2: f64 = c[[0, 1]].powi(2)
            + c[[1, 1]].powi(2)
            + 2.0 * (c[[0, 2]] * c[[0, 0]] + c[[1, 2]] * c[[1, 0]]);
        let a1: f64 = 2.0 * (c[[0, 1]] * c[[0, 0]] + c[[1, 1]] * c[[1, 0]]);
        let a0: f64 = c[[0, 0]].powi(2) + c[[1, 0]].powi(2) - 4.0 * R.powi(2);
        let roots: Roots<f64> = find_roots_quartic(a4, a3, a2, a1, a0);

        let min_root: Option<f64> = match roots {
            Roots::One(values) => smallest_positive_root(&values),
            Roots::Two(values) => smallest_positive_root(&values),
            Roots::Three(values) => smallest_positive_root(&values),
            Roots::Four(values) => smallest_positive_root(&values),
            _ => None,
        };

        if let Some(time_delta) = min_root {
            self.time_delta = time_delta;
        }
    }

    fn apply(&self, balls: &mut Vec<Ball>) {}
}

pub fn smallest_positive_root(roots: &[f64]) -> Option<f64> {
    roots.iter().copied().min_by(|x, y| x.total_cmp(y))
}
