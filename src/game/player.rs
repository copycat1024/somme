pub trait Player {
    type Move;
    fn choose(&self, list: &[Self::Move]) -> usize;
}
