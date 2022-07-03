use crate::ball::{Ball, BallState};
use crate::pool_table::{Cushion, Pocket};
use crate::{DELTA, G, MU};
use ndarray::{array, Array1};

trait Event {
    fn apply(&mut self);
    fn calculate_time_until(&mut self);
    fn get_time_until(&self) -> f64;
    fn mut_compare<T: Event>(next_event: &mut Option<T>, mut other: T) {
        other.calculate_time_until();
        if let Some(e) = next_event {
            if other.get_time_until() < e.get_time_until() {
                *next_event = Some(other);
            }
        } else {
            *next_event = Some(other);
        }
    }
}

struct StopRolling<'a> {
    ball: &'a mut Ball,
    time_delta: f64,
}

impl<'a> StopRolling<'a> {
    fn new(ball: &'a mut Ball) -> Self {
        StopRolling {
            ball,
            time_delta: f64::INFINITY,
        }
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

struct HitPocket<'a> {
    ball: &'a mut Ball,
    pocket: &'a Pocket,
    time_delta: f64,
}

impl<'a> HitPocket<'a> {
    fn new(ball: &'a mut Ball, pocket: &'a Pocket) -> Self {
        HitPocket {
            ball,
            pocket,
            time_delta: f64::INFINITY,
        }
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

struct HitBall<'a> {
    ball: &'a mut Ball,
    other_ball: &'a Ball,
    time_delta: f64,
}

impl<'a> HitBall<'a> {
    fn new(ball: &'a mut Ball, other_ball: &'a Ball) -> Self {
        HitBall {
            ball,
            other_ball,
            time_delta: f64::INFINITY,
        }
    }
}

impl Event for HitBall<'_> {
    fn apply(&mut self) {}

    fn calculate_time_until(&mut self) {}

    fn get_time_until(&self) -> f64 {
        self.time_delta
    }
}
