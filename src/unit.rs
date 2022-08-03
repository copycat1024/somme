use crate::{ActionType, Effect, Loadout};

pub struct Unit {
    pub max_hp: i32,
    pub hp: i32,
    pub att: i32,
    pub moves: Vec<ActionType>,
}

impl Unit {
    pub fn new(cfg: &Loadout) -> Self {
        Self {
            max_hp: cfg.hp,
            hp: cfg.hp,
            att: cfg.att,
            moves: vec![ActionType::Attack],
        }
    }

    pub fn receiver(&mut self, effect: Effect) {
        self.hp -= effect.damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }

    pub fn dead(&self) -> bool {
        self.hp < 1
    }
}
