use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Application error: {0}")]
    Application(String),
}

impl From<opencv::Error> for MyError {
    fn from(error: opencv::Error) -> Self {
        MyError::Application(format!("{error}"))
    }
}

pub const ERROR_OPENING_VIDEO: &str = "Error opening video";