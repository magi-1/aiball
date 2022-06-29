use ndarray::{Array1, array};
use crate::{G, MU};

#[derive(Debug)]
pub enum BallType {
    SOLID,
    EIGHT,
    STRIPED,
    CUE
}

pub struct PoolBalls {
    balls: Vec<Ball>
}

impl PoolBalls {
    pub fn new() -> Vec<Ball> {
        (1..17).map(|i| Ball::new(i)).collect()
    }
}

pub struct Ball {
    pub btype: BallType,
    pub r: Array1<f64>,
    pub v: Array1<f64>
}

impl Ball {

    pub fn new(n: usize) -> Ball { 
        assert!(0 < n && n < 17);
        let btype = match n {
            1..=7 => BallType::SOLID,
            8 => BallType::EIGHT,
            9..=15 => BallType::STRIPED,
            _ => BallType::CUE
        };
        Ball {
            btype, 
            r: array![0.0, 0.0], 
            v: array![0.0, 0.0]
        }
    }

    pub fn mag_v(&self) -> f64 {
        (&self.v.dot(&self.v)).sqrt()
    }

    pub fn p_t(&self, t: f64) -> Array1<f64> {
        &self.r + (self.mag_v()*t-0.5*MU*G*t)*&self.v
    }

    pub fn v_t(&self, t: f64) -> Array1<f64> {
        (self.mag_v()-MU*G*t)*&self.v
    }
}