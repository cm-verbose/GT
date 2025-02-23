#[derive(Debug)]
pub enum TokenType {
  BRACKET,
  COMMENT,
  IDENTIFIER,
  KEYWORD,
  NUMBER,
  OPERATOR,
  STRING,
}

#[derive(Debug)]
pub struct Token {
  pub literal: String,
  pub token_type: TokenType,
}
