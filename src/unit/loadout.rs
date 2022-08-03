use super::{Attr, Position};

pub struct Loadout {
    pub attr: Attr,
    pub pos: Position,
}

impl Loadout {
    pub fn new(team: usize, x: i32, y: i32, attr: &Attr) -> Self {
        Self {
            attr: attr.clone(),
            pos: Position { team, x, y },
        }
    }
}
