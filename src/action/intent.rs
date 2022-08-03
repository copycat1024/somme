use super::ActionKind;

pub struct Intent {
    pub source: usize,
    pub target: usize,
    pub action: ActionKind,
    pub(crate) _lock: (),
}

impl Intent {
    pub(crate) fn new(source: usize, target: usize, action: ActionKind) -> Self {
        Self {
            source,
            target,
            action,
            _lock: (),
        }
    }
}
