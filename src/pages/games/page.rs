use super::game::Game;

#[derive(PartialEq, Eq)]
pub enum Page {
  Home,
  Game(Game),
}
