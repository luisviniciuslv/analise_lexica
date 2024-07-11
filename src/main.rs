use std::rc::Rc;
use std::str::Chars;
use std::iter::Peekable;

// caso não seja espaço em branco, número ou parenteses, retorna falso
fn is_not_reserved(c: &char) -> bool {
    !c.is_whitespace() && !c.is_numeric() && *c != '(' && *c != ')'
}

// Token, que é um enum que pode ser um parenteses esquerdo, direito, número ou identificador
// partialEq e Eq são traits que permitem comparar valores de um tipo
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    LPar,
    RPar,
    Num(u64),
    Id(String),
}

pub struct Lexer<'a> {
    peekable: Peekable<Chars<'a>>, // Peekable é um iterador que permite ver o próximo elemento sem avançar
}

// Implementação do Lexer
impl<'a> Lexer<'a> {

    // Construtor do Lexer
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            peekable: input.chars().peekable(), // chars() retorna um iterador de caracteres, e peekable() transforma em um Peekable
        }
    }

    // Função que pula caracteres que satisfazem a condição do predicado
    fn skip(&mut self, predicate: impl Fn(&char) -> bool) {

        // Enquanto houver um próximo caractere e ele satisfazer a condição do predicado, avança
        while let Some(&c) = self.peekable.peek() {
            if predicate(&c) {
                self.peekable.next();
            } else { // Se não satisfazer a condição, para
                break;
            }
        }
    }

    // Função que acumula caracteres que satisfazem a condição do predicado 
    fn accumulate(&mut self, predicate: impl Fn(&char) -> bool) -> String {
        let mut result = String::new(); // Cria uma string vazia
        while let Some(&c) = self.peekable.peek() { // Enquanto houver um próximo caractere
            if predicate(&c) { // Se o caractere satisfazer a condição do predicado, adiciona na string e avança
                result.push(c);
                self.peekable.next();
            } else {
                break;
            }
        }
        result
    }

    // Função que avança o iterador e retorna o próximo token
    pub fn bump(&mut self) -> Option<Token> {
        match self.peekable.peek()? { // match é um switch case
            '(' => { // Se o próximo caractere for um parenteses esquerdo, avança e retorna um token de parenteses esquerdo
                self.peekable.next();
                Some(Token::LPar)
            }
            ')' => { // o mesmo para parenteses direito
                self.peekable.next();
                Some(Token::RPar)
            }
            ' ' | '\n' | '\t' => { // Se for espaço em branco, pula
                self.skip(|&c| c == ' ' || c == '\n' || c == '\t');
                self.bump()
            }
            digit if digit.is_numeric() => {
                // Se for um número, acumula os números e retorna um token de número
                let num = self.accumulate(|&c| c.is_numeric());
                Some(Token::Num(num.parse().unwrap()))
            }
            _ => {
                // Se não for nenhum dos anteriores, acumula os caracteres que não são reservados e retorna um token de identificador
                let identifier = self.accumulate(is_not_reserved);
                Some(Token::Id(identifier))
            }
        }
    }
}

#[derive(Debug)]
pub enum Expr { // Enum que representa uma expressão, que pode ser um identificador, número ou lista de expressões
    Id(String),
    Num(u64),
    List(Vec<Expr>)
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token> // Token atual, que é um Option<Token>, option é um enum que pode ser Some ou None
}

impl<'a> Parser<'a> {
    // Construtor do Parser
    pub fn new(mut lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            current: lexer.bump(), // Token atual é o próximo token do lexer
            lexer
        }
    }

    pub fn advance(&mut self) { // Função que avança o token atual
        let new = self.lexer.bump(); // Pega o próximo token do lexer
        self.current = new; // Atualiza o token atual
    }

    pub fn eat(&mut self, token: Token) -> Result<Token, String> { // Função que consome um token
        if let Some(current) = &self.current { // Se o token atual não for None
            if *current == token { // Se o token atual for igual ao token que queremos consumir
                self.advance(); // Avança o token atual
                Ok(token) // Retorna o token que foi consumido
            } else { // Se não for igual, retorna um erro
                Err(format!("expected other token {:?}", token))
            }
        } else { // Se o token atual for None, retorna um erro
            Err("expected token but got end of file".to_owned()) // to_owned() transforma a string em um String, que é um tipo de string do Rust que aloca na heap
        } // O Rust tem dois tipos de strings, a &str, que é uma referência para uma string, e a String, que é uma string alocada na heap
    }// A diferença entre as duas é que a &str é imutável e a String é mutável

    pub fn parse_list(&mut self) -> Result<Expr, String> { // Função que faz o parsing de uma lista de expressões, que é uma lista de tokens entre parenteses
        self.eat(Token::LPar)?; // Consome um parenteses esquerdo, se não conseguir, retorna um erro

        let mut expressions = Vec::new(); // Cria um vetor de expressões

        while self.current != Some(Token::RPar) { // Enquanto o token atual não for um parenteses direito
            let expr = self.parse_term()?; // Faz o parsing de uma expressão, que pode ser um identificador, número ou lista de expressões, e retorna um erro se não conseguir
            expressions.push(expr); // Adiciona a expressão no vetor de expressões
        }

        self.eat(Token::RPar)?; // Consome um parenteses direito, se não conseguir, retorna um erro

        Ok(Expr::List(expressions)) // Retorna uma expressão que é uma lista de expressões
    }

    pub fn parse_term(&mut self) -> Result<Expr, String> { // Função que faz o parsing de uma expressão, retorna um erro se não conseguir
        let token = if let Some(token) = &self.current { // Se o token atual não for None
            token
        } else {
            return Err("end of file.".to_owned()) // Se for None, retorna um erro
        };

        match token { 
            Token::LPar => self.parse_list(), // Se o token for um parenteses esquerdo, faz o parsing de uma lista de expressões
            Token::RPar => Err("unexpected right parenthesis".to_owned()), // Se for um parenteses direito, retorna um erro
            Token::Num(n) => { // Se for um número, cria uma expressão de número e avança o token atual
                let expr = Expr::Num(*n); // Cria uma expressão de número
                self.advance(); // Avança o token atual
                Ok(expr) // Retorna a expressão
            },
            Token::Id(id) => { // Se for um identificador, cria uma expressão de identificador e avança o token atual
                let expr = Expr::Id(id.clone()); // Cria uma expressão de identificador
                self.advance(); 
                Ok(expr) // Retorna a expressão
            },
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> { // Função que faz o parsing de várias expressões
        let mut expressions = Vec::new(); 

        while self.current != None { // Enquanto o token atual não for None
            let expr = self.parse_list()?; // Faz o parsing de uma lista de expressões
            expressions.push(expr); // Adiciona a expressão no vetor de expressões
        }

        Ok(expressions) // Retorna o vetor de expressões
    }
}

// Interpreter

pub type IntFun = Rc<dyn Fn(Vec<Value>) -> Result<Value, String>>; // Tipo de função que recebe um vetor de valores e retorna um resultado, 
                                                                   // que pode ser um valor ou um erro

pub enum Value { // Enum que representa um valor, que pode ser um número ou uma função
    Int(u64),
    Func(IntFun),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::Func(_) => "<func>".to_owned(),
        }
    }
}

impl Value {
    pub fn get_int(&self) -> Result<u64, String> {
        match self {
            Value::Int(n) => Ok(*n),
            Value::Func(_) => Err("expected an int".to_owned()),
        }
    }

    pub fn get_fn(&self) -> Result<IntFun, String> {
        match self {
            Value::Int(_) => Err("expected a function".to_owned()),
            Value::Func(n) => Ok(n.clone())
        }
    }
}

pub fn interpret(expr: &Expr) -> Result<Value, String> {
    match expr {
        Expr::Id(n) => {
            if n == "+" {
                Ok(Value::Func(Rc::new(|args| {
                    let mut acc = 0;

                    for arg in args {
                        let num = arg.get_int()?;
                        acc += num;
                    }

                    Ok(Value::Int(acc))
                })))
            } else {
                Err("cannot find the function".to_owned())
            }
        },
        Expr::Num(n) => Ok(Value::Int(*n)),
        Expr::List(l) => {
            if l.is_empty() {
                Err("empty function call bleeeh".to_owned())
            } else {
                let fun = &l[0];
                let args = &l[1..];
                
                let fun_value = interpret(fun)?;

                let mut args_value = Vec::new();

                for arg in args {
                    let arg_value = interpret(arg)?;
                    args_value.push(arg_value);
                }

                let fun = fun_value.get_fn()?;

                fun(args_value)
            }
        }
    }
}

fn main() {
    let input = "(+ 1 (+ 1 1) 3)";
    let lexer = Lexer::new(input);
    let trees = Parser::new(lexer).parse().unwrap();

    for tree in trees {
        let result = interpret(&tree).unwrap();
        println!("{}", result.to_string())
    }
}