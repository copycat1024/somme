use hecs::Entity;

use super::ActionKind;

#[derive(Clone)]
pub struct Intent {
    pub source: Entity,
    pub target: Entity,
    pub action: ActionKind,
}

impl Intent {
    pub(crate) fn new(source: Entity, target: Entity, action: ActionKind) -> Self {
        Self {
            source,
            target,
            action,
        }
    }
}
