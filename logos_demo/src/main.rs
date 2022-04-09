use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex("[a-z][a-zA-Z0-9_]*")]
    Identifier,
    #[regex("[1-9][0-9]*")]
    Integer,
    #[regex("[1-9][0-9]\\.[0-9]*")]
    Float,
    #[regex("(\\+|-)")]
    Op,
    #[token("=")]
    Assign,
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn main() {
    let mut lex = Token::lexer("let sum = 5 + 6");
    loop {
        match lex.next() {
            Some(Token::Identifier) => {
                println!("id {} {:?}", lex.slice(), lex.span())
            }
            Some(Token::Integer) => {
                println!("int {} {:?}", lex.slice(), lex.span())
            }
            Some(Token::Float) => {
                println!("float {} {:?}", lex.slice(), lex.span())
            }
            Some(Token::Op) => {
                println!("op {} {:?}", lex.slice(), lex.span())
            }
            Some(Token::Assign) => {
                println!("assign {} {:?}", lex.slice(), lex.span())
            }
            Some(Token::Error) => {
                println!("err {} {:?}", lex.slice(), lex.span())
            }
            None => break,
        }
    }
}
