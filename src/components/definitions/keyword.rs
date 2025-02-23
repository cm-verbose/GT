#[derive(Debug)]
pub enum Keyword {
  IF,
  ELSE,
  FALSE,
  WHILE,
  VALTRUE,
}

impl Keyword {
  pub fn as_str(&self) -> &'static str {
    match self {
      Keyword::IF => "if",
      Keyword::ELSE => "else",
      Keyword::FALSE => "false",
      Keyword::VALTRUE => "true",
      Keyword::WHILE => "while",
    }
  }

  pub fn iterator() -> std::slice::Iter<'static, Keyword> {
    static KEYWORDS: [Keyword; 5] = [
      Keyword::IF,
      Keyword::ELSE,
      Keyword::FALSE,
      Keyword::VALTRUE,
      Keyword::WHILE,
    ];
    KEYWORDS.iter()
  }
}
