#[derive(Clone, Copy)]
pub struct Unit {
    pub team: usize,
    pub x: i32,
    pub y: i32,
    pub hero: &'static str,
}
