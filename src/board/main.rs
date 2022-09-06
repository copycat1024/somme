use crate::{Action, Attr, Loadout, Result, Rule, Unit};
use hecs::World;

pub struct Board {
    pub turn: u64,
    pub ecs: World,
}

impl Board {
    pub fn new(loadout: &[Loadout], rule: &Rule) -> Result<Self> {
        let mut ecs = World::new();
        for item in loadout {
            ecs.spawn((item.unit, Attr::default()));
        }

        for (_, (unit, attr)) in ecs.query::<(&Unit, &mut Attr)>().iter() {
            *attr = rule.get_attr(unit)?;
        }

        Ok(Self { turn: 0, ecs })
    }

    pub fn accept(&mut self, action: &Action) -> Result<()> {
        let target = action.intent.target;
        let delta = action.effect.delta();

        self.ecs
            .get::<&mut Attr>(target)
            .map(|mut a| a.apply(&delta))?;

        Ok(())
    }
}
