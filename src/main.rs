#[derive(Debug)]
pub enum Token {
  Number(u32),
  Plus,
  Minus,
  Star,
  Slash,
  Name(String)
}

#[derive(Debug)]
pub enum LexState {
  Default,
  Number,
  Name,
  Stop,
}

fn tokens(s: &str) -> Vec<Token> {
  let mut lex_state = LexState::Default;
  let mut number = String::new();
  let mut name = String::new();
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
          Some(c) => {
            name.push(c);
            lex_state = LexState::Name;
          },
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

      LexState::Name => match s.chars().nth(cursor) {

        Some(c) if c.is_ascii_alphabetic() || c.is_ascii_punctuation() => { // Adicionado is_ascii_punctuation para aceitar caracteres especiais pq sim :D
          name.push(c);
          cursor += 1;
          continue 'lex;
        }

        Some(_other) => {
          tokens.push(Token::Name(name.clone()));
          name = String::new();
          lex_state = LexState::Default;
          continue 'lex;
        }

        None => {
          tokens.push(Token::Name(name.clone()));
          lex_state = LexState::Stop;
          continue 'lex;
        }
      },

      LexState::Stop => break,
    }
  }

  tokens
}

fn interpreter(tokens: Vec<Token>) -> u32 { // tentativa falha de implementar a função interpreter
  let mut result = 0;
  let mut operator = Token::Plus;

  for token in tokens {
    match token {
      Token::Number(n) => match operator {
        Token::Plus => result += n,
        Token::Minus => result -= n,
        Token::Star => result *= n,
        Token::Slash => result /= n,
        _ => println!("Invalid operator")
      },
      Token::Plus => operator = Token::Plus,
      Token::Minus => operator = Token::Minus,
      Token::Star => operator = Token::Star,
      Token::Slash => operator = Token::Slash,
      Token::Name(n) => println!("{}", n)
    }
  }

  result
}
fn main() {
  let s = "1 * 9 ola 123 lol removi o panic -1";
  let t = tokens(s);
  println!("{:?}", t);
  let result = interpreter(t);
  println!("{}", result);
}
