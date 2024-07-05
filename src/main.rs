#[derive(Debug, Clone, Copy, PartialEq)] // Eu não entendi o que é isso
pub enum Token {
    Number(u32),
    Plus,
    Minus,
    Star,
    Slash
}

fn main() {
    let s = "11+ 2 * 33";
    let tokens = token(s);
    println!("{:?}", tokens);
}

fn token(s: &str) -> Vec<Token> {
    let mut vec_token = Vec::new();
    let mut s = s.chars();
    let mut current_number = String::new();
    let mut is_number = false;
    
    fn push (token: Token, current_number: &mut String, vec_token: &mut Vec<Token>) {
        if !current_number.is_empty() {
            vec_token.push(Token::Number(current_number.parse().unwrap()));
            current_number.clear();
        } 
        vec_token.push(token);
    }

    while let Some(c) = s.next() {
        is_number = false;
        match c {
            ' ' => { 
                if !current_number.is_empty() {
                    vec_token.push(Token::Number(current_number.parse().unwrap()));
                    current_number.clear();
                }
             }
            '0'..='9' => {
                is_number = true;
                current_number.push(c);
            }
            '+' => {
                push(Token::Plus, &mut current_number, &mut vec_token)
            }
            '-' => {
                push(Token::Minus, &mut current_number, &mut vec_token)
            }
            '*' => {
                push(Token::Star, &mut current_number, &mut vec_token)
            }
            '/' => {
                push(Token::Slash, &mut current_number, &mut vec_token)
            }
            _ => {
                panic!("Invalid character found: {}", c);
            }
        }


        if !is_number && !current_number.is_empty() {
            vec_token.push(Token::Number(current_number.parse().unwrap()));
            current_number.clear();
        }
    }
    
    if !current_number.is_empty() { // Adiciona o último número se houver
        vec_token.push(Token::Number(current_number.parse().unwrap()));
    }

    return vec_token;
}
