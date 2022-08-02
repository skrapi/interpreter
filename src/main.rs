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

impl From<Option<char>> for TokenType {
    fn from(character: Option<char>) -> Self {
        match character {
            Some(char) => match char {
                '=' => TokenType::Assign,
                '+' => TokenType::Plus,
                '(' => TokenType::LeftParenthesis,
                ')' => TokenType::RightParenthesis,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RighBrace,
                ',' => TokenType::Comma,
                ';' => TokenType::Semicolon,
                _ => TokenType::Illegal,
            },
            None => TokenType::EOF,
        }
    }
}

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    character: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            character: None,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read_position >= self.input.len() {
            self.character = None;
        } else {
            self.character = Some(self.input.chars().nth(self.read_position).unwrap());
        }

        self.position = self.read_position;
        self.read_position += 1;

        match self.position >= self.input.len() {
            true => None,
            false => Some(Token {
                token_type: self.character.into(),
                literal: Some(self.character.unwrap().to_string()),
            }),
        }
    }
}

fn basic_test() {
    let input = "=+(){},;";

    let expected_output = vec![
        Token {
            token_type: TokenType::Assign,
            literal: Some('='.to_string()),
        },
        Token {
            token_type: TokenType::Plus,
            literal: Some('+'.to_string()),
        },
        Token {
            token_type: TokenType::LeftParenthesis,
            literal: Some('('.to_string()),
        },
        Token {
            token_type: TokenType::RightParenthesis,
            literal: Some(')'.to_string()),
        },
        Token {
            token_type: TokenType::LeftBrace,
            literal: Some('{'.to_string()),
        },
        Token {
            token_type: TokenType::RighBrace,
            literal: Some('}'.to_string()),
        },
        Token {
            token_type: TokenType::Comma,
            literal: Some(','.to_string()),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: Some(';'.to_string()),
        },
        Token {
            token_type: TokenType::EOF,
            literal: None,
        },
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
        Token {
            token_type: TokenType::Semicolon,
            literal: Some(";".to_string()),
        },
        Token {
            token_type: TokenType::Let,
            literal: Some("let".to_string()),
        },
        Token {
            token_type: TokenType::Indentifier,
            literal: Some("ten".to_string()),
        },
        Token {
            token_type: TokenType::Assign,
            literal: Some("=".to_string()),
        },
        Token {
            token_type: TokenType::Integer,
            literal: Some("10".to_string()),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: Some(";".to_string()),
        },
        Token {
            token_type: TokenType::Let,
            literal: Some("let".to_string()),
        },
        Token {
            token_type: TokenType::Indentifier,
            literal: Some("add".to_string()),
        },
        Token {
            token_type: TokenType::Assign,
            literal: Some("=".to_string()),
        },
        Token {
            token_type: TokenType::Function,
            literal: Some("fn".to_string()),
        },
        Token {
            token_type: TokenType::LeftParenthesis,
            literal: Some("(".to_string()),
        },
        Token {
            token_type: TokenType::Indentifier,
            literal: Some("x".to_string()),
        },
        Token {
            token_type: TokenType::Comma,
            literal: Some(','.to_string()),
        },
        Token {
            token_type: TokenType::Indentifier,
            literal: Some("y".to_string()),
        },
        Token {
            token_type: TokenType::RightParenthesis,
            literal: Some(")".to_string()),
        },
        Token {
            token_type: TokenType::EOF,
            literal: None,
        },
    ];

    let lexer = Lexer::new(input.to_string());

    for (index, token) in lexer.enumerate() {
        println!("{index}, {token:?}");
        assert_eq!(token.token_type, expected_output[index].token_type);
        assert_eq!(token.literal, expected_output[index].literal);
    }

    println!("Success!")
}
