use super::token::{Keywords, Token, TokenType};

pub struct Lexer {
  ptr: usize,
  code: String,
  chars: Vec<char>,
  tokens: Vec<Token>,
}

impl Lexer {
  pub fn ini() -> Self {
    Lexer {
      ptr: 0,
      code: String::new(),
      chars: Vec::new(),
      tokens: Vec::new(),
    }
  }

  /// Lexes a string of code
  pub fn lex(&mut self, code: String) -> &Vec<Token> {
    self.reset();
    self.code = code;
    self.chars = self.code.chars().collect();

    'lex_loop: while self.ptr < self.code.len() && self.chars.get(self.ptr).is_some() {
      let current: char = *self.chars.get(self.ptr).unwrap();
      let curr_string: String = current.to_string();

      match current {
        ';' => self.lex_comment(),
        '+' | '-' | '*' | '/' | '%' | '!' | '=' | '<' | '>' => {
          let op_eq: String = curr_string + "=";

          if let Some(peeked) = self.peek(2) {
            if peeked == op_eq {
              self.add_token(peeked, TokenType::OPERATOR);
              self.ptr += 2;
              continue;
            }
          }
          self.add_token(current.to_string(), TokenType::OPERATOR);
        }

        // Brackets
        '(' | ')' | '[' | ']' | '{' | '}' => {
          self.add_token(curr_string, TokenType::BRACKET);
        }

        '\"' => self.lex_string(),
        _ => {}
      }

      if current.is_digit(10) {
        self.lex_number();
      }

      // identifiers & keywords
      if current.is_ascii_alphabetic() {
        let mut identifier_literal: String = String::new();

        'identifier_loop: while let Some(character) = self.chars.get(self.ptr) {
          if !character.is_alphanumeric() {
            break 'identifier_loop;
          }
          identifier_literal.push(*character);
          self.ptr += 1;
        }

        for keyword in Keywords::iterator() {
          if keyword.as_str() == identifier_literal {
            self.add_token(identifier_literal.clone(), TokenType::KEYWORD);
            continue 'lex_loop;
          }
        }

        self.add_token(identifier_literal, TokenType::IDENTIFIER);
        continue;
      }
      self.ptr += 1;
    }

    return &self.tokens;
  }

  fn lex_comment(&mut self) {
    let mut comment_literal: String = String::new();
    self.ptr += 1;

    while let Some(character) = self.chars.get(self.ptr) {
      if *character == '\n' {
        break;
      };
      comment_literal.push(*character);
      self.ptr += 1;
    }
    self.add_token(comment_literal, TokenType::COMMENT);
  }

  fn lex_number(&mut self) {
    let mut number_literal: String = String::new();

    while let Some(character) = self.chars.get(self.ptr) {
      let num_part: char = *character;
      if num_part == '.' {
        number_literal.push('.');
        self.ptr += 1;
        continue;
      }
      if !character.is_digit(10) {
        break;
      }
      number_literal.push(num_part);
      self.ptr += 1;
    }

    self.add_token(number_literal, TokenType::NUMBER);
  }

  fn lex_string(&mut self) {
    let mut string_literal: String = String::new();

    self.ptr += 1;
    while let Some(character) = self.chars.get(self.ptr) {
      if *character == '\"' {
        break;
      }

      // Escape sequences
      if *character == '\\' {
        if let Some(next_char) = self.chars.get(self.ptr + 1) {
          string_literal.push('\\');

          match *next_char {
            'n' | '\"' | 'r' | '\0' | '\\' => {
              string_literal.push(*next_char);
              self.ptr += 2;
              continue;
            }
            _ => {}
          }
        }
      }

      string_literal.push(*character);
      self.ptr += 1;
    }
    self.add_token(string_literal, TokenType::STRING);
  }

  /// Peeks foward to see a string of length `amount`.
  fn peek(&self, amount: usize) -> Option<String> {
    let start: usize = self.ptr;
    let end: usize = start + amount;

    if end > self.code.len() && start <= self.code.len() {
      None
    } else {
      let part: String = self.code[start..end].to_string();
      Some(part)
    }
  }

  /// Pushes a token to the end of the token list
  fn add_token(&mut self, literal: String, token_type: TokenType) {
    let token = Token {
      literal,
      token_type,
    };
    self.tokens.push(token);
  }

  /// Reset the internal state of the lexer for multiple uses
  fn reset(&mut self) {
    self.ptr = 0;
    self.code = String::new();
    self.chars = Vec::new();
    self.tokens = Vec::new();
  }
}
