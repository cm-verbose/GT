#![allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Keyword {
  CONST,
  IF,
  ELSE,
  VAL_FALSE,
  VAL_TRUE,
  WHILE,
}

impl Keyword {
  pub fn as_str(&self) -> &'static str {
    match self {
      Keyword::CONST => "const",
      Keyword::IF => "if",
      Keyword::ELSE => "else",
      Keyword::VAL_FALSE => "false",
      Keyword::VAL_TRUE => "true",
      Keyword::WHILE => "while",
    }
  }

  pub fn iterator() -> std::slice::Iter<'static, Keyword> {
    static KEYWORDS: [Keyword; 6] = [
      Keyword::CONST,
      Keyword::IF,
      Keyword::ELSE,
      Keyword::VAL_FALSE,
      Keyword::VAL_TRUE,
      Keyword::WHILE,
    ];
    KEYWORDS.iter()
  }
}
