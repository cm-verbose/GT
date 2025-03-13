#[derive(Debug)]
pub enum TokenType {
  Bracket,
  Comment,
  Identifier,
  Keyword,
  Number,
  Operator,
  String,
}

#[derive(Debug)]
pub struct Token {
  pub literal: String,
  pub kind: TokenType,
}
