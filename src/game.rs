use crate::events::*;
use crate::SimObjects;

struct Game {
    sim_objects: SimObjects,
}

impl Game {
    pub fn new() -> Self {
        Self {
            sim_objects: SimObjects::new(),
        }
    }
    fn make_move(&mut self) {}

    fn evaluate_game_state(&mut self) {}

    pub fn get_next_event(&self) -> Option<EventEnum> {
        let objects = &self.sim_objects;
        let mut next_event = EventEnum::NullEvent(NullEvent::new());
        for ball_id in 0..objects.pool_balls.num_balls() {
            for pocket_id in 0..objects.pool_table.num_pockets() {
                let mut event = EventEnum::HitPocket(HitPocket::new(ball_id, pocket_id));
                event.calculate_time_until(objects);
                next_event.mut_compare(event);
            }

            for cushion_id in 0..objects.pool_table.num_cushions() {
                let mut event = EventEnum::HitCushion(HitCushion::new(ball_id, cushion_id));
                event.calculate_time_until(objects);
                next_event.mut_compare(event);            }

            for other_ball_id in 0..objects.pool_balls.num_balls() {
                let mut event = EventEnum::HitBall(HitBall::new(ball_id, other_ball_id));
                event.calculate_time_until(objects);
                next_event.mut_compare(event);            }
        }
        match next_event {
            EventEnum::NullEvent(_) => None,
            _ => Some(next_event),
        }
    }

    fn play_turn(&mut self) {
        self.make_move();
        while let Some(event) = self.get_next_event() {
            event.apply(&mut self.sim_objects);
        }
        self.evaluate_game_state();
    }
}
