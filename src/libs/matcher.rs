use libs::ast::AST;

pub struct Matcher {
  ast: AST,

}

impl Matcher {
  pub fn new(ast: AST) -> Matcher {
    Matcher {
      ast: ast
    }
  }

  pub fn match_<'a>(&self, text: &'a str) -> Result<&'a str, String> {
    match self.match_re(&text, &self.ast) {
      Ok("") => Ok(text),
      Err(e) => Err(e),
      Ok(s) => Err(format!("failed: {}", s))
    }
  }

  fn match_re<'a>(&self, text: &'a str, ast: &AST) -> Result<&'a str, String> {
    match *ast {
      AST::Union { ref left, ref right } => self.match_union(text, &left, &right),
      AST::Concat { ref left, ref right } => self.match_concat(text, &left, &right),
      AST::Closure(ref x) => self.match_closure(text, &x),
      AST::Paren(ref x) => self.match_paren(text, &x),
      AST::Character(x) => self.match_character(text, x)
    }
  }

  fn match_union<'a>(&self, text: &'a str, left: &AST, right: &AST) -> Result<&'a str, String> {
    if let Ok(x) = self.match_re(text, left) {
      return Ok(x)
    }
    if let Ok(x) = self.match_re(text, right) {
      return Ok(x)
    }
    Err(format!("failed: {} {} {}", left, right, text))
  }

  fn match_concat<'a>(&self, text: &'a str, left: &AST, right: &AST) -> Result<&'a str, String> {
    if let Ok(s) = self.match_re(text, left) {
      if let Ok(ss) = self.match_re(s, right) {
        return Ok(ss)
      }
    }
    Err(format!("failed: {} {} {}", left, right, text))
  }

  fn match_closure<'a>(&self, text: &'a str, node: &AST) -> Result<&'a str, String> {
    let mut s = text;
    while let Ok(ss) = self.match_re(s, node) {
      s = ss
    }
    Ok(s)
  }

  fn match_paren<'a>(&self, text: &'a str, node: &AST) -> Result<&'a str, String> {
    self.match_re(text, node)
  }

  fn match_character<'a>(&self, text: &'a str, ch: char) -> Result<&'a str, String> {
    if text.starts_with(ch) {
      return Ok(&text[1..])
    }
    Err(format!("failed: {} {}", ch, text))
  }
}
