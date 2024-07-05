#[derive(Debug, Clone, Copy, PartialEq)] // Eu não entendi o que é isso
pub enum Token {
    Number(u32),
    Plus,
    Minus,
    Star,
    Slash
}

fn main() {
    let s = "1 + 2 * 3";
    let tokens = token(s);
    println!("{:?}", tokens);
}

fn token(s: &str) -> Vec<Token> {
    let mut vec_token = Vec::new();
    let mut s = s.chars();

    while let Some(c) = s.next() {
        println!("{}", c);
        match c {
            ' ' => {
                // Skip
            }
            '0'..='9' => {
                vec_token.push(Token::Number(c.to_digit(10).unwrap()));
            }
            '+' => {
                vec_token.push(Token::Plus );
            }
            '-' => {
                vec_token.push(Token::Minus );
            }
            '*' => {
                vec_token.push(Token::Star );
            }
            '/' => {
                vec_token.push(Token::Slash );
            }
            _ => {
                panic!("Invalid character found: {}", c);
            }
        }
    }
    return vec_token
}
