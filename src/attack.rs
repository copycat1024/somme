use crate::{Effect, Unit};

pub fn effect(source: &Unit, _target: &Unit) -> Effect {
    Effect { damage: source.att }
}
