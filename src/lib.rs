mod engine;
mod game;

pub fn run() {
    let game = game::lowrez_game::LowRezGame::new(60);
    engine::start_game(Box::new(game));
}
