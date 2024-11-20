use std::{
  fmt::Display,
  path::{Path, PathBuf},
};

use podstru_derive::{Builder, Fields};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceLoc {
  path: Option<PathBuf>,
  line: Option<usize>,
  col: Option<usize>,
}

impl SourceLoc {
  pub fn new<P: AsRef<Path>>(path: P, line: usize, column: usize) -> Self {
    Self {
      path: Some(path.as_ref().to_path_buf()),
      line: Some(line),
      col: Some(column),
    }
  }

  pub fn path(&self) -> Option<&PathBuf> {
    self.path.as_ref()
  }

  pub fn path_mut(&mut self) -> &mut Option<PathBuf> {
    &mut self.path
  }

  pub fn line(&self) -> Option<usize> {
    self.line
  }

  pub fn line_mut(&mut self) -> &mut Option<usize> {
    &mut self.line
  }

  pub fn column(&self) -> Option<usize> {
    self.col
  }

  pub fn column_mut(&mut self) -> &mut Option<usize> {
    &mut self.col
  }
}

impl Display for SourceLoc {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let path = match self.path.to_string_lossy().is_empty() {
      true => String::new(),
      false => format!("{}", self.path.display()),
    };
    let line = self.line.to_string();
    let col = self.col.to_string();
    let parts: [&str; 3] = match (self.line, self.col) {
      (0, 0) => [path.as_str(), "", ""],
      (_, 0) => [path.as_str(), line.as_str(), ""],
      (0, _) => [path.as_str(), "1", col.as_str()],
      (_, _) => [path.as_str(), line.as_str(), col.as_str()],
    };
    write!(f, "{}", parts.join(":"))
  }
}
