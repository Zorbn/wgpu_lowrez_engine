mod engine;
mod lowrez_game;

pub fn run() {
    let game = lowrez_game::LowRezGame::new(60);
    engine::start_game(Box::new(game));
}
