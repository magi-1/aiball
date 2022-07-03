use crate::{DELTA, G, MU};
use ndarray::{array, Array1};

#[derive(Debug)]
pub enum BallType {
    SOLID,
    EIGHT,
    STRIPED,
    CUE,
}

#[derive(Debug, PartialEq)]
pub enum BallState {
    STATIONARY,
    MOVING,
    POCKETED,
}

pub struct Ball {
    pub btype: BallType,
    pub bstate: BallState,
    pub r: Array1<f64>,
    pub v: Array1<f64>,
    pub mag_v: f64,
}

impl Ball {
    pub fn new(btype: BallType) -> Self {
        Ball {
            btype,
            bstate: BallState::STATIONARY,
            r: array![0.0, 0.0],
            v: array![0.0, 0.0],
            mag_v: 0.0,
        }
    }

    pub fn update_state(&mut self, time_delta: f64) {
        self.r = self.r_t(time_delta);
        self.v = self.v_t(time_delta);
        self.mag_v = (&self.v.dot(&self.v)).sqrt();

        if self.mag_v < DELTA {
            self.bstate = BallState::STATIONARY;
        } else {
            self.bstate = BallState::MOVING;
        }
    }

    pub fn r_t(&self, t: f64) -> Array1<f64> {
        &self.r + (self.mag_v * t - 0.5 * MU * G * t) * &self.v
    }

    pub fn v_t(&self, t: f64) -> Array1<f64> {
        (self.mag_v - MU * G * t) * &self.v
    }

    pub fn reset(&mut self) {
        self.r = array![0.0, 0.0];
        self.v = array![0.0, 0.0];
        self.mag_v = 0.0;
        self.bstate = BallState::STATIONARY;
    }

    fn is_pocketed(&self) -> bool {
        self.bstate == BallState::POCKETED
    }

    fn is_moving(&self) -> bool {
        self.bstate == BallState::MOVING
    }
}
