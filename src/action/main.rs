use std::fmt::Debug;

use super::{ActionKind, Effect, Intent};

pub struct Action {
    pub intent: Intent,
    pub effect: Effect,
}

impl Action {
    pub(crate) fn new(intent: &Intent, effect: Effect) -> Self {
        Self {
            intent: intent.clone(),
            effect,
        }
    }
}

impl Debug for Action {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let Self {
            intent:
                Intent {
                    action,
                    source,
                    target,
                    ..
                },
            effect: Effect { dmg },
        } = self;
        let kind = match action {
            ActionKind::Attack => "attack",
        };
        write!(fmt, "Action {}({:?}->{:?}) {{", kind, source, target)?;
        if *dmg > 0 {
            write!(fmt, " damage({})", dmg)?;
        }
        write!(fmt, " }}")?;
        Ok(())
    }
}
