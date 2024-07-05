#[derive(Debug, Clone, Copy, PartialEq)] // Eu não entendi o que é isso
pub enum Token {
    Number(u32),
    Plus,
    Minus,
    Star,
    Slash
}

fn main() {
    let s = "11 + 2 * 3";
    let tokens = token(s);
    println!("{:?}", tokens);
}

fn token(s: &str) -> Vec<Token> {
    let mut vec_token = Vec::new();
    let mut s = s.chars();
    let mut current_number = String::new();

    while let Some(c) = s.next() {
        match c {
            ' ' => {
                if !current_number.is_empty() {
                    vec_token.push(Token::Number(current_number.parse().unwrap()));
                    current_number.clear();
                }
            }
            '0'..='9' => {
                current_number.push(c);
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

    if !current_number.is_empty() { // Adiciona o último número se houver
        vec_token.push(Token::Number(current_number.parse().unwrap()));
    }
    
    return vec_token
}
