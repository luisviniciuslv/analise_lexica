#[derive(Debug)]
pub enum Token {
  Number(u32),
  Plus,
  Minus,
  Star,
  Slash,
  Name(String),
  Function,
}

#[derive(Debug)]
pub enum LexState {
  Default,
  Number,
  Name,
  Stop,
  Error(char),
}

#[derive(Debug)]
pub enum DefaultFunctions {
  Print,
  Sum,
  Unknown,
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
          }

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
          lex_state = LexState::Error(_other);
          continue 'lex;
        }

        None => {
          tokens.push(Token::Number(number.parse().unwrap()));
          lex_state = LexState::Stop;
          continue 'lex;
        }
      },

      LexState::Name => match s.chars().nth(cursor) {
        Some(c) if c.is_ascii_alphabetic() => {
          // Adicionado is_ascii_punctuation para aceitar caracteres especiais pq sim :D
          name.push(c);
          cursor += 1;
          continue 'lex;
        }

        Some(c) => {
          tokens.push(Token::Name(name.clone()));
          lex_state = LexState::Error(c);
          continue 'lex;
        }

        None => {
          tokens.push(Token::Name(name.clone()));
          lex_state = LexState::Stop;
          continue 'lex;
        }
      },

      LexState::Stop => break,

      LexState::Error(_c) => {
        name = String::new();
        number = String::new();
        cursor += 1;
        lex_state = LexState::Default;
      }
    }
  }

  tokens
}

fn some_stack(tokens: Vec<Token>) {
  let mut stack: Vec<i32> = Vec::new();

  for token in tokens {
    match token {
      Token::Number(n) => stack.push(n as i32),

      Token::Minus | Token::Plus | Token::Slash | Token::Star => {
        if stack.len() >= 2 {
          let right = stack.pop().unwrap(); // unwrap é (tenho certeza que existe um número aquim, mas se eu estiver errado para ap orra toda)
          let left = stack.pop().unwrap();

          let result = match token {
            Token::Plus => left + right,
            Token::Minus => left - right,
            Token::Star => left * right,
            Token::Slash => left / right,
            _ => unreachable!(), // Verifiquei tudo
          };

          stack.push(result);
        }
      }

      Token::Name (name) => {
        let func = environment(name.to_string().as_str());

        match func {
          DefaultFunctions::Print => {
            println!("{:?}", stack);
          }

          DefaultFunctions::Sum => {
            let sum = stack.iter().sum();
            stack.push(sum);
            stack.pop().unwrap_or(0);
          }

          DefaultFunctions::Unknown => {
            println!("Unknown function: {}", name);
          }
        }
      }
      _ => {} // outros tokens
    }
  }
  
}

fn environment(token: &str) -> DefaultFunctions {
  if  token == "print" {
    DefaultFunctions::Print
  } else if token == "sum" {
    DefaultFunctions::Sum
  } else {
    // Add an else block that evaluates to the expected type
    DefaultFunctions::Unknown // Or any other appropriate value
  }
}

fn main() {
  let s = "2 3 4 sum";
  let t = tokens(s);

  let some = some_stack(t);
  println!("{:?}", some)
}