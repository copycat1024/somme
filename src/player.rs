pub trait Player {
    type Move;

    fn choose<'a>(&self, list: &'a [Self::Move]) -> &'a Self::Move;
}
