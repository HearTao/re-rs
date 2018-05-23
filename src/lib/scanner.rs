use lib::ast::Token;

pub struct Scanner {
  chars: Vec<char>,
  pos: usize
}

impl Scanner {
  pub fn new(text: &str) -> Scanner {
    Scanner {
      chars: text.chars().collect(),
      pos: 0
    }
  }

  pub fn curr(&mut self) -> Option<char> {
    if self.pos < self.chars.len() {
      Some(self.chars[self.pos])
    } else {
      None
    }
  }

  pub fn incr(&mut self) -> usize {
    self.pos += 1;
    self.pos
  }

  pub fn next_token(&mut self) -> Option<Token> {
    let token = match self.curr() {
      Some('*') => Some(Token::Star),
      Some('|') => Some(Token::Bar),
      Some('(') => Some(Token::OpenParen),
      Some(')') => Some(Token::CloseParen),
      Some(c) => Some(Token::Character(c)),
      None => None
    };
    self.incr();
    token
  }
}

impl Iterator for Scanner {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
      self.next_token()
  }
}