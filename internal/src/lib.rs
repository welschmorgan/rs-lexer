pub mod err;
pub mod loc;

pub use err::*;
pub use loc::*;

use std::{
  borrow::Cow,
  collections::VecDeque,
  fmt::Display,
  io::BufRead,
  ops::{Deref, DerefMut},
  path::{Path, PathBuf},
  str::FromStr,
};

use podstru_derive::{Builder, Ctor, Fields};
use podstru_internal::Builder;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Fields, Builder, Ctor)]
pub struct Region {
  start: SourceLoc,
  end: SourceLoc,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MappedRegion<'a> {
  pub region: Region,
  pub content: Cow<'a, str>,
}

impl<'a> MappedRegion<'a> {
  pub fn new(region: Region, content: Cow<'a, str>) -> Self {
    Self { region, content }
  }

  pub fn region(&self) -> &Region {
    &self.region
  }

  pub fn region_mut(&mut self) -> &mut Region {
    &mut self.region
  }

  pub fn content(&'a self) -> &Cow<'a, str> {
    &self.content
  }

  pub fn content_mut(&'a mut self) -> &mut Cow<'a, str> {
    &mut self.content
  }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<'text> {
  pub region: MappedRegion<'text>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokens<'text>(VecDeque<Token<'text>>);

impl<'text> Tokens<'text> {
  pub fn new() -> Self {
    Self(VecDeque::new())
  }
}

impl<'text> Deref for Tokens<'text> {
  type Target = VecDeque<Token<'text>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'text> DerefMut for Tokens<'text> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

pub struct Cursor<'text> {
  /// textual content reference
  pub data: &'text str,
  /// current source code offset
  pub index: usize,
  /// current source code location
  pub loc: SourceLoc,
  /// an optional length, it won't be able to read past it
  pub len: Option<usize>,
}

pub struct Lexer<'text> {
  text: Option<&'text str>,
  tokens: Tokens<'text>,
}

pub const LEXER_BLOCK_SIZE: usize = 255;

impl<'text> Lexer<'text> {
  pub fn new() -> Self {
    Self {
      text: None,
      tokens: Tokens::new(),
    }
  }

  pub fn from_file<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
    let path = path.as_ref().to_path_buf();
    let f = std::fs::File::open(&path).map_err(|e| {
      fail!(
        ErrorKind::IO,
        format!("{}: failed to open file", path.display())
      )
    })?;
    Self::from_reader(f, Some(path.clone()))
  }

  pub fn from_reader<R: std::io::Read>(r: R, path: Option<PathBuf>) -> crate::Result<Self> {
    let blocks: Vec<u8> = Vec::new();
    let mut block: [u8; LEXER_BLOCK_SIZE] = [0u8; LEXER_BLOCK_SIZE];
    loop {
      let block_size = r.read(&mut block)?;
      blocks.extend_from_slice(&block[0..block_size]);
      if block_size != LEXER_BLOCK_SIZE {
        break;
      }
    }
    let content = std::str::from_utf8(&blocks)?;
    let mut lexer = Self {
      text: Some(content),
      tokens: Tokens::new(),
    };
    lexer.analyse()?;
    Ok(lexer)
  }

  pub fn analyse(&mut self) -> crate::Result<&'text Tokens<'text>> {
    todo!("Lexer::analyse")
  }
}

impl<'text> FromStr for Lexer<'text> {
  type Err = crate::Error;

  fn from_str(s: &str) -> crate::Result<Self> {
    let mut curs = std::io::Cursor::new(s.as_ref());
    Self::from_reader(&mut curs, Some(path.cloned()))
  }
}

#[cfg(test)]
mod tests {
  use crate::Lexer;

  #[test]
  fn test() {
    let lex = Lexer::new();
  }
}
