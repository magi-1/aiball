use crate::ball::{Ball, BallState};
use crate::pool_table::{Cushion, Pocket};
use crate::{DELTA, G, MU};
use ambassador::{delegatable_trait, Delegate};
use ndarray::{array, Array1};

#[delegatable_trait]
pub trait Event {
    fn apply(&mut self);
    fn calculate_time_until(&mut self);
    fn get_time_until(&self) -> f64;
}

#[derive(Delegate)]
#[delegate(Event)]
pub enum EventEnum<'a> {
    StopRolling(StopRolling<'a>),
    HitBall(HitBall<'a>),
    HitCushion(HitCushion<'a>),
    HitPocket(HitPocket<'a>),
    NullEvent(NullEvent),
}

pub struct NullEvent {
    time_delta: f64,
}

impl<'a> NullEvent {
    pub fn new() -> Self {
        NullEvent {
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for NullEvent {
    fn apply(&mut self) {}

    fn calculate_time_until(&mut self) {}

    fn get_time_until(&self) -> f64 {
        self.time_delta
    }
}

pub struct StopRolling<'a> {
    ball: &'a mut Ball,
    time_delta: f64,
}

impl<'a> StopRolling<'a> {
    pub fn new(ball: &'a mut Ball) -> Self {
        let mut event = StopRolling {
            ball,
            time_delta: f64::INFINITY,
        };
        event.calculate_time_until();
        event
    }
}

impl Event for StopRolling<'_> {
    fn apply(&mut self) {
        self.ball.update_state(self.time_delta)
    }

    fn calculate_time_until(&mut self) {
        self.time_delta = self.ball.mag_v / (MU * G);
    }

    fn get_time_until(&self) -> f64 {
        self.time_delta
    }
}

pub struct HitPocket<'a> {
    ball: &'a mut Ball,
    pocket: &'a Pocket,
    time_delta: f64,
}

impl<'a> HitPocket<'a> {
    pub fn new(ball: &'a mut Ball, pocket: &'a Pocket) -> Self {
        let mut event = HitPocket {
            ball,
            pocket,
            time_delta: f64::INFINITY,
        };
        event.calculate_time_until();
        event
    }
}

impl Event for HitPocket<'_> {
    fn apply(&mut self) {
        self.ball.reset();
        self.ball.bstate = BallState::POCKETED;
    }

    fn calculate_time_until(&mut self) {
        self.time_delta = self.ball.mag_v / (MU * G);
    }

    fn get_time_until(&self) -> f64 {
        self.time_delta
    }
}

pub struct HitCushion<'a> {
    ball: &'a mut Ball,
    cushion: &'a Cushion,
    time_delta: f64,
}

impl<'a> HitCushion<'a> {
    pub fn new(ball: &'a mut Ball, cushion: &'a Cushion) -> Self {
        let mut event = HitCushion {
            ball,
            cushion,
            time_delta: f64::INFINITY,
        };
        event.calculate_time_until();
        event
    }
}

impl Event for HitCushion<'_> {
    fn apply(&mut self) {}

    fn calculate_time_until(&mut self) {}

    fn get_time_until(&self) -> f64 {
        self.time_delta
    }
}

pub struct HitBall<'a> {
    ball: &'a mut Ball,
    other_ball: &'a mut Ball,
    time_delta: f64,
}

impl<'a> HitBall<'a> {
    pub fn new(ball: &'a mut Ball, other_ball: &'a mut Ball) -> Self {
        let mut event = HitBall {
            ball,
            other_ball,
            time_delta: f64::INFINITY,
        };
        event.calculate_time_until();
        event
    }
}

impl Event for HitBall<'_> {
    fn apply(&mut self) {}

    fn calculate_time_until(&mut self) {}

    fn get_time_until(&self) -> f64 {
        self.time_delta
    }
}
