use std::fmt;

#[derive(Clone, Copy)]
pub enum Token {
  Character(char),
  Star,
  Bar,
  OpenParen,
  CloseParen
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Token::Star => write!(f, "[ Star ]"),
           Token::Bar => write!(f, "[ Bar ]"),
           Token::OpenParen => write!(f, "[ OpenParen ]"),
           Token::CloseParen => write!(f, "[ CloseParen ]"),
           Token::Character(c) => write!(f, "[ Character {} ]", c),
       }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        match (*self, *other) {
            (Token::Character(a), Token::Character(b)) => a == b,
            (Token::Star, Token::Star) => true,
            (Token::Bar, Token::Bar) => true,
            (Token::OpenParen, Token::OpenParen) => true,
            (Token::CloseParen, Token::CloseParen) => true,
            _ => false,
        }
    }
}

pub enum AST {
  Concat { left: Box<AST>, right: Box<AST> },
  Union { left: Box<AST>, right: Box<AST> },
  Closure(Box<AST>),
  Paren(Box<AST>),
  Character(char)
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           AST::Union { ref left, ref right } => write!(f, "Union ( left: {}, right: {} )", left, right),
           AST::Concat { ref left, ref right } => write!(f, "Concat ( left: {}, right: {} )", left, right),
           AST::Closure(ref x) => write!(f, "Closure ( {} )", x),
           AST::Character(ref c) => write!(f, "Character ( {} )", c),
           AST::Paren(ref x) => write!(f, "Paren ( {} )", x)
       }
    }
}
