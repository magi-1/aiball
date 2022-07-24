use aiball::game::Game;

fn main() {
    let mut game: Game = Game::new();
    game.play_turn(-3.1459 / 2.0, 1.0);
}
