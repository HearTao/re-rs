use lib::ast::Token;
use lib::ast::AST;

pub struct Parser {
  tokens: Vec<Token>,
  pos: usize
}

impl Parser {
  pub fn new<T>(iter: T) -> Parser where T: Iterator<Item = Token> {
    Parser {
      tokens: iter.collect(),
      pos: 0
    }
  }

  pub fn curr(&self) -> Option<Token> {
    self.lookahead(Some(0))
  }

  pub fn lookahead(&self, index: Option<usize>) -> Option<Token> {
    let idx = index.unwrap_or(1);
    if self.pos + idx < self.tokens.len() {
      Some(self.tokens[self.pos + idx])
    } else {
      None
    }
  }

  pub fn advance(&mut self) {
    self.pos += 1
  }

  pub fn parse(&mut self) -> Option<AST> {
    let token = self.parse_simple();
    if let Some(Token::Bar) = self.curr() {
      self.advance();
      let right = self.parse_simple();
      return match (token, right) {
        (Some(l), Some(r)) => Some(
          AST::Union { 
            left: Box::new(l),
            right: Box::new(r)
          }),
        (_, _) => None
      }
    }
    token
  }

  fn parse_simple(&mut self) -> Option<AST> {
    let token = self.parse_basic();
    if let Some(x) = token {
      return match (Some(x), self.parse_simple()) {
        (Some(l), Some(r)) => {
          Some(
            AST::Concat {
              left: Box::new(l),
              right: Box::new(r)
            }
          )
        },
        (Some(l), None) => Some(l),
        (_, _) => None
      }
    }
    token
  }

  fn parse_basic(&mut self) -> Option<AST> {
    let token = self.parse_factor();
    match (token, self.curr()) {
      (Some(a), Some(Token::Star)) => {
        self.advance();
        Some(
          AST::Closure(Box::new(a))
        )
      },
      (Some(a), _) => Some(a),
      (_, _) => None
    }
  }

  fn parse_factor(&mut self) -> Option<AST> {
    match self.curr() {
      Some(Token::OpenParen) => {
        self.advance();
        match (self.parse(), self.curr()) {
          (Some(x), Some(Token::CloseParen)) => {
            self.advance();
            Some(AST::Paren(Box::new(x)))
          },
          (_, _) => None
        }
      },
      Some(Token::Character(c)) => {
        self.advance();
        Some(AST::Character(c))
      },
      _ => None
    }
  }
}