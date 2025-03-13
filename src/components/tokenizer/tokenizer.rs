use super::{
  error_reporter::Reporter,
  keyword::Keyword,
  token::{Token, TokenType},
};

/// Reads the input in order to produce a series of tokens. Can err when an
/// invalid sequence of characters is read.
pub struct Tokenizer {
  chars: Vec<char>,
  line: usize,
  ptr: usize,
  tokens: Vec<Token>,
}

impl Tokenizer {
  pub fn new() -> Self {
    Tokenizer {
      chars: Vec::new(),
      line: 1,
      ptr: 0,
      tokens: Vec::new(),
    }
  }

  pub fn tokenize(&mut self, code: String) -> Result<&Vec<Token>, String> {
    self.reset();
    self.chars = code.chars().collect();

    return self.traverse_chars();
  }

  /// Traverse input characters to identify characters delimiting the start of
  /// any valid token
  fn traverse_chars(&mut self) -> Result<&Vec<Token>, String> {
    while let Some(current_char) = self.chars.get(self.ptr) {
      let current = *current_char;

      self.tokenize_operator_brackets(current);
      match current {
        '#' => self.tokenize_comment(),
        '\"' => self.tokenize_string()?,
        _ => {}
      }

      if current.is_ascii_digit() {
        self.tokenize_number()?
      }

      if current.is_ascii_alphabetic() || current == '_' {
        self.tokenize_identifier();
      }
      self.ptr += 1;
    }
    return Ok(&self.tokens);
  }

  /// Tokenises identifiers and keywords (because they start with letters)
  fn tokenize_identifier(&mut self) {
    let mut literal: String = String::new();

    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;

      if !curr.is_alphanumeric() && curr != '_' {
        break;
      }
      literal.push(curr);
      self.ptr += 1;
    }

    let mut is_keyword: bool = false;
    for keyword in Keyword::iterator() {
      if literal.as_str() == keyword.as_str() {
        is_keyword = true;
      }
    }

    if is_keyword {
      self.add_token(literal, TokenType::Keyword);
    } else {
      self.add_token(literal, TokenType::Identifier);
    }
    self.ptr -= 1;
  }

  /// Tokenize a comment
  fn tokenize_comment(&mut self) {
    let mut literal: String = String::new();

    self.ptr += 1; // Skip comment token
    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;

      if curr == '\n' {
        self.line += 1;
        break;
      }
      literal.push(curr);
      self.ptr += 1;
    }
    self.add_token(literal, TokenType::Comment);
  }

  // Tokenize numbers : integers and floating point numbers
  fn tokenize_number(&mut self) -> Result<(), String> {
    if let Some(sequence) = self.peek(2) {
      match sequence.as_str() {
        "0b" => self.handle_bin_repr(),
        "0o" => self.handle_oct_repr(),
        "0x" => self.handle_hex_repr(),
        _ => self.handle_dec_repr(),
      }
    } else {
      self.handle_dec_repr()
    }
  }

  /// Handles the binary representation of a number
  fn handle_bin_repr(&mut self) -> Result<(), String> {
    let mut literal: String = String::from("0b");
    self.ptr += 2;

    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;
      if curr != '0' && curr != '1' {
        break;
      }
      literal.push(curr);
      self.ptr += 1;
    }

    if literal.len() == 2 {
      let message: String = format!("Incomplete binary expression : {}", literal);
      return Err(Reporter::wrap_error(message));
    }
    self.ptr -= 1;
    Ok(self.add_token(literal, TokenType::Number))
  }

  fn handle_oct_repr(&mut self) -> Result<(), String> {
    let mut literal: String = String::from("0o");
    self.ptr += 2;

    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;
      if !['1', '2', '3', '4', '5', '6', '7', '0'].contains(&curr) {
        break;
      }
      literal.push(curr);
      self.ptr += 1;
    }

    if literal.len() == 2 {
      let message: String = format!("Incomplete octal expression : {}", literal);
      return Err(Reporter::wrap_error(message));
    }
    self.ptr -= 1;
    Ok(self.add_token(literal, TokenType::Number))
  }

  /// Handles the hexadecimal representation of a number
  fn handle_hex_repr(&mut self) -> Result<(), String> {
    let mut literal: String = String::from("0x");
    self.ptr += 2;
    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;

      if !curr.is_ascii_hexdigit() {
        break;
      }
      literal.push(curr);
      self.ptr += 1;
    }

    if literal.len() == 2 {
      let message: String = format!("Incomplete hexadecimal expression : {}", literal);
      return Err(Reporter::wrap_error(message));
    }
    self.ptr -= 1;
    Ok(self.add_token(literal, TokenType::Number))
  }

  /// Handles the decimal representation of a number
  fn handle_dec_repr(&mut self) -> Result<(), String> {
    let mut literal: String = String::new();
    let mut point_count: i32 = 0;

    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;

      if !curr.is_ascii_digit() && curr != '.' {
        break;
      }

      if curr == '.' {
        literal.push(curr);
        self.ptr += 1;
        if point_count < 2 {
          point_count += 1;
        }
        continue;
      }
      literal.push(curr);
      self.ptr += 1;
    }

    if point_count == 2 {
      let message: String = format!("Expression \"{}\" has multiple dots", literal);
      return Err(Reporter::wrap_error(message));
    }
    self.ptr -= 1;
    Ok(self.add_token(literal, TokenType::Number))
  }

  fn tokenize_operator_brackets(&mut self, current: char) {
    match current {
      '+' | '-' | '*' | '/' | '%' | '!' | '<' | '>' | '=' => {
        let operator_equals: String = current.to_string() + "=";
        let peeked: Option<String> = self.peek(2);

        if peeked.is_some() && peeked.unwrap() == operator_equals {
          self.ptr += 1;
          self.add_token(operator_equals, TokenType::Operator);
        } else {
          self.add_token(current.to_string(), TokenType::Operator);
        }
      }

      '(' | ')' | '[' | ']' | '{' | '}' => {
        self.add_token(current.to_string(), TokenType::Bracket);
      }
      _ => {}
    }
  }

  /// Tokenize a string, including character escape sequences
  fn tokenize_string(&mut self) -> Result<(), String> {
    let mut literal: String = String::new();
    let initial_line: usize = self.line;
    self.ptr += 1;

    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;
      if curr == '\"' {
        break;
      }

      match curr {
        '\n' => self.line += 1,
        '\\' => {
          if let Some(next) = self.chars.get(self.ptr + 1) {
            literal.push('\\');
            match *next {
              'b' | 'f' | 'n' | 'r' | 't' | '0' | '\"' | '\\' => {
                literal.push(*next);
                self.ptr += 2;
                continue;
              }
              _ => {}
            }
          }
        }
        _ => {}
      }

      literal.push(curr);
      self.ptr += 1;
    }

    if self.ptr == self.chars.len() {
      let message: String = format!(
        "[Line {}-{}] : Unterminated string",
        initial_line, self.line
      );
      return Err(Reporter::wrap_error(message));
    }

    self.add_token(literal, TokenType::String);
    Ok(())
  }

  /// Looks fowards in the sequence of characters without incrementing the pointer
  fn peek(&self, amount: usize) -> Option<String> {
    let start = self.ptr;
    let end = start + amount;

    if end > self.chars.len() {
      None
    } else {
      let part: String = self.chars[start..end].iter().collect();
      Some(part)
    }
  }

  // Pushes a token to the end of a list
  fn add_token(&mut self, literal: String, kind: TokenType) {
    self.tokens.push(Token { literal, kind });
  }

  /// Resets the internal state of the tokenizer, for when it is used multiple
  /// times.  
  fn reset(&mut self) {
    self.chars = Vec::new();
    self.line = 1;
    self.ptr = 0;
    self.tokens = Vec::new();
  }
}
