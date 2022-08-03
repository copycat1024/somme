use crate::Attr;

#[derive(Copy, Clone)]
pub struct Effect {
    pub dmg: i32,
}

impl Effect {
    pub fn delta(&self) -> Attr {
        Attr::from([("hp", -self.dmg)])
    }
}
