use crate::events::*;
use crate::ball::{Ball};

use crate::SimObjects;

pub struct Game {
    sim_objects: SimObjects,
}

impl Game {
    pub fn new() -> Self {
        Self {
            sim_objects: SimObjects::new(),
        }
    }
    pub fn make_move(&mut self) {
        let cue: &mut Ball = self.sim_objects.get_mut_cue();
        cue.hit(-3.1459/2.0, 50.0);
    }

    fn evaluate_game_state(&mut self) {

    }

    fn get_next_event(&self) -> Option<EventEnum> {
        let objects = &self.sim_objects;
        let mut next_event = EventEnum::default();
        for ball_id in 0..objects.num_balls() {

            let mut event = EventEnum::StopRolling(StopRolling::new(ball_id));
            event.calculate_time_until(objects);
            next_event.mut_compare(event);

            for pocket_id in 0..objects.num_pockets() {
                let mut event = EventEnum::HitPocket(HitPocket::new(ball_id, pocket_id));
                event.calculate_time_until(objects);
                next_event.mut_compare(event);
            }

            for cushion_id in 0..objects.num_cushions() {
                let mut event = EventEnum::HitCushion(HitCushion::new(ball_id, cushion_id));
                event.calculate_time_until(objects);
                next_event.mut_compare(event);
            }

            for other_ball_id in 0..objects.num_balls() {
                if other_ball_id < ball_id {
                    let mut event = EventEnum::HitBall(HitBall::new(ball_id, other_ball_id));
                    event.calculate_time_until(objects);
                    next_event.mut_compare(event);
                }
            }
        }
        match next_event {
            EventEnum::NullEvent(_) => None,
            _ => Some(next_event),
        }
    }

    pub fn play_turn(&mut self) {
        self.make_move();
        let mut iters = 0;
        while let Some(event) = self.get_next_event() {
            iters += 1;
            println!("{}", iters);
            self.sim_objects.apply_event(event);
        }
        println!("Move Complete!!!");
        self.evaluate_game_state();
    }

    fn num_stationary_balls(&self) -> usize {
        let mut sum: usize = 0;
        for b in &self.sim_objects.balls {
            if !b.is_moving() {
                sum += 1;
            }
        }
        sum
    }
}
