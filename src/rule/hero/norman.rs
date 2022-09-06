use super::Hero;
use crate::Attr;

#[derive(Default)]
pub struct Norman {}

impl Hero for Norman {
    fn name(&self) -> &'static str {
        "norman"
    }

    fn attr(&self) -> Attr {
        Attr::new(100, 20)
    }
}
