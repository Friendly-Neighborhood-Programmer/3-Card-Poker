mod card;
mod deck;
mod game;
mod hand;
mod player;

fn main() {
    game::init_app().unwrap();
}
