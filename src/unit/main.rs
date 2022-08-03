use super::{Attr, Loadout, Position};
use crate::{Action, ActionKind, Effect, Intent};

pub struct Unit {
    attr: Attr,
    pub pos: Position,
    pub moves: Vec<ActionKind>,
}

impl Unit {
    pub fn new(load: &Loadout) -> Self {
        Self {
            attr: load.attr.clone(),
            pos: load.pos,
            moves: vec![ActionKind::Attack],
        }
    }

    pub fn act(&self, intent: Intent, target: &Self) -> Action {
        use super::attack;

        let effect = match intent.action {
            ActionKind::Attack => attack::effect(self, target),
        };

        Action::new(intent, effect)
    }

    pub fn receiver(&mut self, effect: Effect) {
        let delta = effect.delta();
        self.attr.apply(&delta);
    }

    pub fn dead(&self) -> bool {
        self.attr("hp") < 1
    }

    pub fn attr(&self, key: &'static str) -> i32 {
        self.attr.get(key)
    }
}
