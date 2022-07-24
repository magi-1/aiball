use crate::ball::Ball;
use crate::events::*;

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

    fn update_state(&mut self) {}

    fn get_next_event(&self) -> Option<EventEnum> {
        let objects = &self.sim_objects;
        let mut next_event = EventEnum::default();
        for ball_id in 0..objects.num_balls() {

            if objects.get_ball(ball_id).is_moving() {
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
            
        }
        match next_event {
            EventEnum::NullEvent(_) => None,
            _ => Some(next_event),
        }
    }

    pub fn play_turn(&mut self, phi: f64, force: f64) {
        // Applying force to cue
        let cue: &mut Ball = self.sim_objects.get_mut_cue();
        cue.hit(phi, force);

        // Running simulations
        while let Some(event) = self.get_next_event() {
            for b in &self.sim_objects.balls {
                println!("{:?}", b);
            }
            
            self.sim_objects.apply_event(event);
        }

        // Updating game state
        self.update_state();
    }

}
