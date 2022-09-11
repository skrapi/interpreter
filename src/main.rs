#[derive(PartialEq, Debug)]
enum TokenType {
    Illegal,
    EOF,
    // Identifiers and literals
    Indentifier,
    Integer,
    // Operators
    Assign,
    Plus,
    // Delimiters
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RighBrace,
    // Keywords
    Function,
    Let,
}

fn main() {
    println!("Running test");
    basic_test();
    advanced_test();
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    literal: Option<String>,
}

impl Token {
    fn new(token_type: TokenType, literal: Option<String>) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}

impl From<Option<&str>> for TokenType {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(value) => match value {
                "=" => TokenType::Assign,
                "+" => TokenType::Plus,
                "(" => TokenType::LeftParenthesis,
                ")" => TokenType::RightParenthesis,
                "{" => TokenType::LeftBrace,
                "}" => TokenType::RighBrace,
                "," => TokenType::Comma,
                ";" => TokenType::Semicolon,
                "fn" => TokenType::Function,
                "let" => TokenType::Let,
                _ => {
                    if value
                        .chars()
                        .all(|char| char.is_alphabetic() || char == '_')
                    {
                        TokenType::Indentifier
                    } else {
                        TokenType::Illegal
                    }
                }
            },
            None => TokenType::EOF,
        }
    }
}

struct Lexer {
    input: String,
    start_of_current_token: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            start_of_current_token: 0,
            read_position: 0,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token_literal = None;
        self.start_of_current_token = self.read_position;
        self.read_position += 1;

        if self.read_position >= self.input.len() {
            token_literal = None;
        } else {
            while self.read_position < self.input.len()
                && self
                    .input
                    .chars()
                    .nth(self.read_position)
                    .unwrap()
                    .is_alphabetic()
            {
                println!("{}", self.read_position);
                self.read_position += 1;
            }

            token_literal = Some(
                &self.input[self
                    .input
                    .char_indices()
                    .nth(self.start_of_current_token)
                    .unwrap()
                    .0
                    ..self.input.char_indices().nth(self.read_position).unwrap().0],
            );
        }

        match self.start_of_current_token >= self.input.len() {
            true => None,
            false => Some(Token {
                token_type: token_literal.into(),
                literal: Some(token_literal.unwrap().to_string()),
            }),
        }
    }
}

fn basic_test() {
    let input = "=+(){},;";

    let expected_output = vec![
        Token::new(TokenType::Assign, Some('='.to_string())),
        Token::new(TokenType::Plus, Some('+'.to_string())),
        Token::new(TokenType::LeftParenthesis, Some('('.to_string())),
        Token::new(TokenType::RightParenthesis, Some(')'.to_string())),
        Token::new(TokenType::LeftBrace, Some('{'.to_string())),
        Token::new(TokenType::RighBrace, Some('}'.to_string())),
        Token::new(TokenType::Comma, Some(','.to_string())),
        Token::new(TokenType::Semicolon, Some(';'.to_string())),
        Token::new(TokenType::EOF, None),
    ];

    let lexer = Lexer::new(input.to_string());

    for (index, token) in lexer.enumerate() {
        println!("{index}, {token:?}");
        assert_eq!(token.token_type, expected_output[index].token_type);
        assert_eq!(token.literal, expected_output[index].literal);
    }

    println!("Success!")
}

fn advanced_test() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
x + y;
};

let result = add(five, ten);
";
    let expected_output = vec![
        Token::new(TokenType::Let, Some("let".to_string())),
        Token::new(TokenType::Indentifier, Some("five".to_string())),
        Token::new(TokenType::Assign, Some("=".to_string())),
        Token::new(TokenType::Integer, Some("5".to_string())),
        Token::new(TokenType::Semicolon, Some(";".to_string())),
        Token::new(TokenType::Let, Some("let".to_string())),
        Token::new(TokenType::Indentifier, Some("ten".to_string())),
        Token::new(TokenType::Assign, Some("=".to_string())),
        Token::new(TokenType::Integer, Some("10".to_string())),
        Token::new(TokenType::Semicolon, Some(";".to_string())),
        Token::new(TokenType::Let, Some("let".to_string())),
        Token::new(TokenType::Indentifier, Some("add".to_string())),
        Token::new(TokenType::Assign, Some("=".to_string())),
        Token::new(TokenType::Function, Some("fn".to_string())),
        Token::new(TokenType::LeftParenthesis, Some("(".to_string())),
        Token::new(TokenType::Indentifier, Some("x".to_string())),
        Token::new(TokenType::Comma, Some(','.to_string())),
        Token::new(TokenType::Indentifier, Some("y".to_string())),
        Token::new(TokenType::RightParenthesis, Some(")".to_string())),
        Token::new(TokenType::EOF, None),
    ];

    let lexer = Lexer::new(input.to_string());

    for (index, token) in lexer.enumerate() {
        println!("{index}, {token:?}");
        assert_eq!(token.token_type, expected_output[index].token_type);
        assert_eq!(token.literal, expected_output[index].literal);
    }

    println!("Success!")
}
