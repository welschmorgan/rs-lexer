use std::{fmt::Display, str::Utf8Error, sync::Arc};

use crate::SourceLoc;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {
  IO,
  Encoding,
  Unknown,
}

#[derive(Clone, Debug)]
pub struct Error {
  kind: ErrorKind,
  message: Option<String>,
  cause: Option<Arc<dyn std::error::Error>>,
  location: SourceLoc,
}

impl Default for Error {
  fn default() -> Self {
    Self {
      kind: ErrorKind::Unknown,
      message: None,
      cause: None,
      location: SourceLoc::default(),
    }
  }
}

impl Error {
  pub fn new(
    k: ErrorKind,
    message: Option<String>,
    cause: Option<Arc<dyn std::error::Error + 'static>>,
    loc: SourceLoc,
  ) -> Self {
    Self {
      kind: k,
      message,
      cause,
      location: loc,
    }
  }

  pub fn with_kind(mut self, k: ErrorKind) -> Self {
    self.kind = k;
    self
  }

  pub fn with_message<M: AsRef<str>>(mut self, m: M) -> Self {
    self.message = Some(m.as_ref().to_string());
    self
  }

  pub fn with_cause<E: std::error::Error + 'static>(mut self, c: E) -> Self {
    self.cause = Some(Arc::new(c));
    self
  }

  pub fn with_location(mut self, l: SourceLoc) -> Self {
    self.location = l;
    self
  }

  pub fn kind(&self) -> ErrorKind {
    self.kind
  }

  pub fn kind_str(&self) -> &str {
    match self.kind {
      ErrorKind::IO => "I/O",
      ErrorKind::Unknown => "unkown",
    }
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn cause(&self) -> Option<&Arc<dyn std::error::Error>> {
    self.cause.as_ref()
  }

  pub fn location(&self) -> &SourceLoc {
    &self.location
  }
}

#[macro_export]
macro_rules! fail {
  ($kind: expr, $msg: expr) => {
    $crate::err::Error::default()
      .with_kind($kind)
      .with_message($msg)
      .with_location($crate::SourceLoc::new(
        file!(),
        line!() as usize,
        column!() as usize,
      ))
  };
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}{}",
      self.kind_str(),
      match self.message() {
        Some(msg) => format!(": {}", msg),
        None => String::new(),
      },
      match self.cause() {
        Some(cause) => format!("\nCaused By: {}", cause.to_string()),
        None => String::new(),
      }
    )
  }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
  fn from(value: std::io::Error) -> Self {
    fail!(ErrorKind::IO, value.to_string())
  }
}

impl From<Utf8Error> for Error {
  fn from(value: Utf8Error) -> Self {
    fail!(ErrorKind::Encoding, value.to_string())
  }
}
