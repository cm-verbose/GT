use super::token::Token;

/// Parses a series of tokens to generate an abstract syntax tree
pub struct Parser {
  tokens: Vec<Token>,
}

impl Parser {
  pub fn new() -> Self {
    Parser { tokens: Vec::new() }
  }

  /// Generate abstract syntax tree from a given set of tokens obtained from
  /// the lexer.
  pub fn parse(&mut self, tokens: &Vec<Token>) {
    self.reset();
    for token in tokens {
      println!("[{:?}] {}", token.kind, token.literal);
    }
  }

  /// Reset the parser's internal state for multiple reuse
  pub fn reset(&mut self) {
    self.tokens = Vec::new();
  }
}
