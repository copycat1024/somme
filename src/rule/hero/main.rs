use crate::{ActionKind, Attr};

pub trait Hero {
    fn name(&self) -> &'static str;
    fn attr(&self) -> Attr;

    fn moves(&self) -> &[ActionKind] {
        &[ActionKind::Attack]
    }
}
