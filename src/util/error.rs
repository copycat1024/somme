use hecs::ComponentError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidUnit,
    InvalidIntent,
    HeroNotFound,
}

impl From<ComponentError> for Error {
    fn from(_: ComponentError) -> Self {
        Self::InvalidUnit
    }
}
