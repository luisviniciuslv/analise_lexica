#[derive(Debug)]
pub enum Token {
  Number(u32),
  Plus,
  Minus,
  Star,
  Slash,
}

#[derive(Debug)]
pub enum LexState {
  Default,
  Number,
  Error(char),
  Stop,
}

fn tokens(s: &str) -> Vec<Token> {
  let mut lex_state = LexState::Default;
  let mut number = String::new();
  let mut tokens = Vec::new();
  let mut cursor = 0;

  'lex: loop {
    match &lex_state {
      LexState::Default => {
        match s.chars().nth(cursor) {
          Some(' ') => {}
          Some('+') => tokens.push(Token::Plus),
          Some('-') => tokens.push(Token::Minus),
          Some('*') => tokens.push(Token::Star),
          Some('/') => tokens.push(Token::Slash),
          Some(c) if c.is_ascii_digit() => {
            number.push(c);
            lex_state = LexState::Number;
          }
          Some(c) => lex_state = LexState::Error(c),
          None => lex_state = LexState::Stop,
        }
        cursor += 1;
      }
      LexState::Number => match s.chars().nth(cursor) {
        Some(c) if c.is_ascii_digit() => {
          number.push(c);
          cursor += 1;
          continue 'lex;
        }
        Some(_other) => {
          tokens.push(Token::Number(number.parse().unwrap()));
          number = String::new();
          lex_state = LexState::Default;
          continue 'lex;
        }
        None => {
          tokens.push(Token::Number(number.parse().unwrap()));
          number = String::new();
          lex_state = LexState::Stop;
          continue 'lex;
        }
      },
      LexState::Error(c) => panic!("Unexpected character '{c}'"),
      LexState::Stop => break,
    }
  }

  tokens
}

fn main() {
  println!("{:?}", tokens("42 3 + 5 - 2 * 4  /"));
}