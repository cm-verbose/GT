use super::keyword::Keyword;
use super::token::{Token, TokenType};

/// A lexer that returns a sequence of tokens when using the
/// `lex()` method
pub struct Lexer {
  ptr: usize,
  line: usize,
  chars: Vec<char>,
  tokens: Vec<Token>,
}

impl Lexer {
  pub fn new() -> Self {
    Lexer {
      ptr: 0,
      line: 1,
      chars: Vec::new(),
      tokens: Vec::new(),
    }
  }

  /// Lex provided source code
  pub fn lex(&mut self, code: String) -> Result<&Vec<Token>, String> {
    self.reset();
    self.chars = code.chars().collect();

    return self.pass_chars();
  }

  /// Iterate through characters to identify tokens
  fn pass_chars(&mut self) -> Result<&Vec<Token>, String> {
    while let Some(current_char) = self.chars.get(self.ptr) {
      let current: char = *current_char;

      self.try_lex_operators_brackets(current);
      match current {
        ';' => self.lex_comments(),
        '\"' => self.lex_string()?,
        '\n' => self.line += 1,
        _ => {}
      }

      if current.is_ascii_digit() {
        self.lex_number()?
      }

      if current.is_alphabetic() {
        self.lex_identifier_keywords()?
      }
      self.ptr += 1;
    }
    return Ok(&self.tokens);
  }

  /// Attempts to lex operators and brackets
  fn try_lex_operators_brackets(&mut self, current: char) {
    match current {
      '+' | '-' | '*' | '/' | '%' | '!' | '<' | '>' | '=' => {
        let operator_equals: String = current.to_string() + "=";
        let peeked = self.peek(2);

        if peeked.is_some() && peeked.unwrap() == operator_equals {
          self.ptr += 2;
          self.add_token(operator_equals, TokenType::OPERATOR);
        } else {
          self.add_token(current.to_string(), TokenType::OPERATOR);
        }
      }

      '(' | ')' | '[' | ']' | '{' | '}' => {
        self.add_token(current.to_string(), TokenType::BRACKET);
      }
      _ => {}
    }
  }

  /// Lexes a comment (starts with ';')
  fn lex_comments(&mut self) {
    let mut literal: String = String::new();

    self.ptr += 1;

    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;
      if curr == '\n' {
        self.line += 1;
        break;
      }
      literal.push(curr);
      self.ptr += 1;
    }
    self.add_token(literal, TokenType::COMMENT);
  }

  /// Lexes both identifiers and keywords (since they start with letters)
  fn lex_identifier_keywords(&mut self) -> Result<(), String> {
    let mut literal: String = String::new();

    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;

      if !curr.is_alphanumeric() {
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
      self.add_token(literal, TokenType::KEYWORD)
    } else {
      self.add_token(literal, TokenType::IDENTIFIER);
    }
    Ok(())
  }

  /// Lexes a number
  fn lex_number(&mut self) -> Result<(), String> {
    let mut literal: String = String::new();
    let mut dots: i32 = 0;

    while let Some(current) = self.chars.get(self.ptr) {
      let curr: char = *current;

      if !curr.is_ascii_digit() {
        if !curr.is_ascii_whitespace() {
          let message: String = format!(
            "Invalid character found after digit sequence \"{}\" : \"{}\"",
            literal, curr
          );
          let error = self.format_error(message);
          return Err(error);
        }
        break;
      }

      if curr == '.' {
        literal.push(curr);

        if dots <= 2 {
          dots += 1;
        }
      }
      literal.push(curr);
      self.ptr += 1;
    }

    if dots >= 2 {
      let message: String = format!("Expression \"{}\" has multiple dots", literal);
      let error: String = self.format_error(message);
      return Err(error);
    }
    Ok(self.add_token(literal, TokenType::NUMBER))
  }

  /// Lexes a string
  fn lex_string(&mut self) -> Result<(), String> {
    let mut literal: String = String::new();
    let initial_line = self.line;

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
              'f' | 'n' | 'r' | 't' | '0' | '\"' | '\\' => {
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

      if self.ptr == self.chars.len() - 1 {
        let message: String = format!("Unterminated string starting from line {}", initial_line);
        let error: String = self.format_error(message);
        return Err(error);
      }

      literal.push(curr);
      self.ptr += 1;
    }
    Ok(self.add_token(literal, TokenType::STRING))
  }

  /// Adds a token to the token list
  fn add_token(&mut self, literal: String, token_type: TokenType) {
    let token = Token {
      literal,
      token_type,
    };
    self.tokens.push(token);
  }

  /// Looks forwards in the characters
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

  /// Reset lexer to its initial state for multiple uses
  fn reset(&mut self) {
    self.ptr = 0;
    self.line = 1;
  }

  /// Format an error
  fn format_error(&self, message: String) -> String {
    let error_message = format!("[line {}]: {}", self.line, message);
    return error_message;
  }
}
