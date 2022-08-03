use crate::{Action, Effect, Error, Flow, Intent, Loadout, Unit};

pub struct Board {
    pub units: Vec<Unit>,
    pub log: Vec<Action>,
}

impl Board {
    pub fn new(load: &[&Loadout]) -> Self {
        Self {
            units: load.iter().map(|l| Unit::new(l)).collect(),
            log: Vec::new(),
        }
    }

    pub fn list_move(&self, id: usize) -> Result<Vec<Intent>, Error> {
        Ok(self
            .unit(id)?
            .moves
            .iter()
            .map(|a| Intent::new(id, (id + 1) % 2, *a))
            .collect())
    }

    pub fn accept(&mut self, intent: &Intent) -> Result<Flow, Error> {
        self.log.clear();

        let action = self.get_action(*intent)?;
        self.apply(intent.target, action.effect)?;
        self.log.push(action);

        self.end()
    }
}

impl Board {
    fn unit(&self, id: usize) -> Result<&Unit, Error> {
        self.units.get(id).ok_or(Error::InvalidUnit)
    }

    fn apply(&mut self, id: usize, effect: Effect) -> Result<(), Error> {
        self.units
            .get_mut(id)
            .map(|a| a.receiver(effect))
            .ok_or(Error::InvalidUnit)
    }

    fn get_action(&self, intent: Intent) -> Result<Action, Error> {
        let source = self.unit(intent.source)?;
        let target = self.unit(intent.target)?;
        Ok(Action::act(intent, source, target))
    }

    fn end(&self) -> Result<Flow, Error> {
        if let Some((i, _)) = self
            .units
            .iter()
            .enumerate()
            .find(|(_, units)| units.dead())
        {
            Ok(Flow::Dead(i))
        } else {
            Ok(Flow::Continue)
        }
    }
}
