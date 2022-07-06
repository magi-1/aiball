use ambassador::{delegatable_trait, Delegate};

use crate::SimObjects;

#[delegatable_trait]
pub trait Event {
    fn calculate_time_until(&self, objects: &SimObjects);
    fn get_time_until(&self) -> f64;
    fn apply(&self, objects: &mut SimObjects);
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

    fn calculate_time_until(&self, objects: &SimObjects) {}

    fn apply(&self, objects: &mut SimObjects) {}
}

pub struct StopRolling {
    ball_id: usize,
    time_delta: f64,
}

impl StopRolling {
    pub fn new(ball_id: usize) -> Self {
        Self {
            ball_id: ball_id,
            time_delta: 0.0,
        }
    }
}

impl Event for StopRolling {
    fn get_time_until(&self) -> f64 {
        self.time_delta
    }

    fn calculate_time_until(&self, objects: &SimObjects) {}

    fn apply(&self, objects: &mut SimObjects) {}
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

    fn calculate_time_until(&self, objects: &SimObjects) {}

    fn apply(&self, objects: &mut SimObjects) {}
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

    fn calculate_time_until(&self, objects: &SimObjects) {}

    fn apply(&self, objects: &mut SimObjects) {}
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

    fn calculate_time_until(&self, objects: &SimObjects) {}

    fn apply(&self, objects: &mut SimObjects) {}
}
