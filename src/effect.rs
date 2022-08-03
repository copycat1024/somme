use crate::{ActionType, Unit};

#[derive(Copy, Clone)]
pub struct Effect {
    pub damage: i32,
}

impl Effect {
    pub fn new(act: ActionType, source: &Unit, target: &Unit) -> Effect {
        use crate::attack;

        match act {
            ActionType::Attack => attack::effect(source, target),
        }
    }
}
