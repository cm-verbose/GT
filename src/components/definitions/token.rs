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
pub enum Keywords {
  IF,
  ELSE,
  FALSE,
  WHILE,
  VALTRUE,
}

impl Keywords {
  pub fn as_str(&self) -> &'static str {
    match self {
      Keywords::IF => "if",
      Keywords::ELSE => "else",
      Keywords::FALSE => "false",
      Keywords::VALTRUE => "true",
      Keywords::WHILE => "while",
    }
  }

  pub fn iterator() -> std::slice::Iter<'static, Keywords> {
    static KEYWORDS: [Keywords; 5] = [
      Keywords::IF,
      Keywords::ELSE,
      Keywords::FALSE,
      Keywords::VALTRUE,
      Keywords::WHILE,
    ];
    KEYWORDS.iter()
  }
}

#[derive(Debug)]
pub struct Token {
  pub literal: String,
  pub token_type: TokenType,
}
