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
    test();
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    literal: Option<char>,
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
                literal: self.character,
            }),
        }
    }
}

fn test() {
    let input = "=+(){},;";

    let expected_output = vec![
        Token {
            token_type: TokenType::Assign,
            literal: Some('='),
        },
        Token {
            token_type: TokenType::Plus,
            literal: Some('+'),
        },
        Token {
            token_type: TokenType::LeftParenthesis,
            literal: Some('('),
        },
        Token {
            token_type: TokenType::RightParenthesis,
            literal: Some(')'),
        },
        Token {
            token_type: TokenType::LeftBrace,
            literal: Some('{'),
        },
        Token {
            token_type: TokenType::RighBrace,
            literal: Some('}'),
        },
        Token {
            token_type: TokenType::Comma,
            literal: Some(','),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: Some(';'),
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
