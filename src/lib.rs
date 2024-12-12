pub mod game;
pub mod ai;
pub use game::Game;
pub use ai::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut game = crate::Game::new();
        crate::ai::negamax_move(&mut game, 11, 0);
    }
}