use crate::{unit::Unit, effect::Effect};

#[derive(Copy, Clone)]
pub struct Intent {
    pub source: usize,
    pub target: usize,
    pub action: ActionType,
}

impl Intent {
    pub fn new(source: usize, target: usize, action: ActionType) -> Self {
        Self {
            source,
            target,
            action,
        }
    }
}

#[derive(Copy, Clone)]
pub enum ActionType {
    Attack,
}

pub struct Action {
    pub intent: Intent,
    pub effect: Effect,
}

impl Action {
    pub fn act(intent: Intent, source: &Unit, target: &Unit) -> Self {
        Self {
            intent,
            effect: Effect::new(intent.action, source, target),
        }
    }
}
