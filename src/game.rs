use crate::{Board, GameView, Intent, Loadout, Player, Turn};

pub enum Flow {
    Continue,
    Dead(usize),
}

pub enum Error {
    InvalidUnit,
}

pub struct Game {
    turn: Turn,
    board: Board,
}

impl Game {
    pub fn new(cfg: &[&Loadout]) -> Self {
        Self {
            turn: Turn::default(),
            board: Board::new(cfg),
        }
    }

    pub fn play<P: Player<Move = Intent>>(&mut self, player: &mut P) -> Result<Flow, Error> {
        let player_id = self.turn.player();
        let moves = self.board.list_move(player_id)?;
        let intent = player.choose(&moves);

        self.board.accept(intent)
    }

    pub fn view(&self) -> GameView<'_> {
        GameView::new(&self.board.units, &self.board.log)
    }
}
