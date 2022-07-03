use crate::events::*;
use crate::pool_balls;
use crate::table;


enum GameState {}

struct Game {
    balls: Vec<Balls>,
    pockets: Vec<Pocket>,
    cushions: Vec<Cusion>,
    events: Vec<impl Event>,
}

impl Game {

    fn new() -> Game{
        Game {
            balls: pool_balls::rack();
            pockets: pool_table::init_pockets();
            cushions: pool_table::init_cushions();
        }
    }

    fn make_move(&mut self) {}

    fn get_next_event(&self) -> Option<impl Event> {
        let mut next_event: Option<impl Event> = None;
        for ball in balls {

            if !ball.is_pocketed() {
                
                if ball.is_moving(){

                    let b_event = StopRolling::new(ball);
                    Event::mut_compare(&mut next_event, b_event);
                    
                    for pocket in self.pockets {
                        let p_event = HitPocket::new(ball, pocket);
                        Event::mut_compare(&mut next_event, p_event);
                    }

                    for cushion in self.cushions {
                        let c_event = HitCushion::new(ball, cushion);
                        Event::mut_compare(&mut next_event, c_event);
                    }
                }  
            }
            
        }
        next_event
    }

    fn step_sim(&mut self) {
        while let Some(event) = self.get_next_event() {
            event.apply();
            for ball in self.balls {
                ball.update_state(event.time_delta);
            }
        }
    }
    
    fn set_game_state(&mut self) {
        /*
        This doesnt update things about the balls, but it reads the balls and sets game level 
        descriptors so that users know who is up, what balls are viable, etc. 
            - current_player
            - game_state 
        */
    }

    fn play_turn(&mut self) {
        self.make_move()
        self.step_sim()
        self.set_game_state();
    }

    
}