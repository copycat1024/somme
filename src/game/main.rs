use crate::{Action, Board, GameView, Intent, Loadout, Player, Result, Rule};

pub enum Flow {
    Continue,
    Dead(usize),
}

pub struct Game {
    board: Board,
    rule: Rule,
}

impl Game {
    pub fn new(load: &[Loadout]) -> Result<Self> {
        let rule = Rule::default();
        let board = Board::new(load, &rule)?;
        Ok(Self { board, rule })
    }

    pub fn play<P: Player<Move = Intent>>(
        &mut self,
        player: &mut P,
    ) -> Result<(Flow, Vec<Action>)> {
        let Self { board, rule } = self;
        rule.play_turn(board, |_| player)
    }

    pub fn view(&self) -> GameView<'_> {
        GameView::new(&self.board.ecs)
    }
}
