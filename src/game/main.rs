use super::Turn;
use crate::{Action, Board, GameView, Intent, Loadout, Player, State};

pub enum Flow {
    Continue,
    Dead(usize),
}

pub enum Error {
    InvalidUnit,
    InvalidIntent,
}

pub struct Game {
    turn: Turn,
    board: Board,
}

impl Game {
    pub fn new(cfg: &[Loadout]) -> Self {
        Self {
            turn: Turn::default(),
            board: Board::new(cfg),
        }
    }

    pub fn play<P: Player<Move = Intent>>(
        &mut self,
        player: &mut P,
    ) -> Result<(Flow, Vec<Action>), Error> {
        let player_id = self.turn.player();
        let mut moves = self.board.list_move(player_id)?;
        let intent = player.choose(&moves);
        let i = index_of_ref(&moves, intent).ok_or(Error::InvalidIntent)?;

        self.turn.step(&());
        self.board.accept(moves.remove(i))
    }

    pub fn view(&self) -> GameView<'_> {
        GameView::new(&self.board.units)
    }
}

fn index_of_ref<T>(v: &Vec<T>, r: &T) -> Option<usize> {
    v.iter().position(|i| std::ptr::eq(i, r))
}
