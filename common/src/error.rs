use core::{
  error::Error,
  fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum ServerError {
  Initialization(String),
}

impl Display for ServerError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      ServerError::Initialization(msg) => write!(f, "Initialization error: {msg}"),
    }
  }
}

impl Error for ServerError {}

pub type ServerResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;
