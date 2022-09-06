use hecs::Entity;
use std::collections::HashMap;

use super::{hero::Norman, Hero, Turn};
use crate::{Action, Attr, Board, Effect, Error, Flow, Intent, Player, Result, Unit};

pub struct Rule {
    turn: Turn,
    hero: HashMap<&'static str, Box<dyn Hero>>,
}

impl Rule {
    pub fn get_active(&self, board: &Board) -> Result<(usize, Entity)> {
        let (e, _) = board
            .ecs
            .query::<()>()
            .iter()
            .next()
            .ok_or(Error::InvalidUnit)?;
        Ok((self.turn.player(), e))
    }

    pub fn get_move_list(&self, board: &Board, id: Entity) -> Result<Vec<Intent>> {
        let unit = board.ecs.get::<&Unit>(id)?;
        let hero = self.hero.get(unit.hero).ok_or(Error::HeroNotFound)?;
        Ok(hero
            .moves()
            .iter()
            .map(|a| Intent::new(id, id, *a))
            .collect())
    }

    pub fn play_turn<'a, FP>(
        &mut self,
        board: &mut Board,
        player_fn: FP,
    ) -> Result<(Flow, Vec<Action>)>
    where
        FP: FnOnce(usize) -> &'a dyn Player<Move = Intent>,
    {
        use crate::State;

        let mut log = Vec::new();

        let (player_id, unit_id) = self.get_active(board)?;
        let player = player_fn(player_id);
        let move_list = self.get_move_list(board, unit_id)?;

        let intent_id = player.choose(&move_list);
        let intent = move_list.get(intent_id).ok_or(Error::InvalidIntent)?;
        let action = self.get_action(&board, &intent)?;

        self.turn.step(&());
        board.accept(&action)?;
        log.push(action);

        Ok((self.end(board), log))
    }

    pub fn get_attr(&self, unit: &Unit) -> Result<Attr> {
        let hero = self.hero.get(unit.hero).ok_or(Error::HeroNotFound)?;
        Ok(hero.attr())
    }

    fn get_action(&self, board: &Board, intent: &Intent) -> Result<Action> {
        let source = board.ecs.get::<&Attr>(intent.source)?;
        let effect = Effect {
            dmg: source.get("att"),
        };
        Ok(Action::new(intent, effect))
    }

    fn end(&self, board: &Board) -> Flow {
        if let Some(_) = board
            .ecs
            .query::<(&Unit, &Attr)>()
            .iter()
            .find(|(_, (_, attr))| attr.get("chp") > 0)
        {
            Flow::Dead(0)
        } else {
            Flow::Continue
        }
    }

    fn add_hero<H: Hero + Default + 'static>(&mut self) {
        let hero = H::default();
        self.hero.insert(hero.name(), Box::new(hero));
    }
}

impl Default for Rule {
    fn default() -> Self {
        let mut me = Self {
            hero: HashMap::new(),
            turn: Turn::default(),
        };
        me.add_hero::<Norman>();
        me
    }
}
