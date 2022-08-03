pub trait State: 'static {
    type Step;
    fn step(&mut self, step: &Self::Step);
}
