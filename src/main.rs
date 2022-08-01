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

struct Token {
    token_type: TokenType,
    literal: char,
}

impl From<char> for TokenType {
    fn from(character: char) -> Self {
        match character.to_string().as_str() {
            "=" => TokenType::Assign,
            "+" => TokenType::Plus,
            "(" => TokenType::LeftParenthesis,
            ")" => TokenType::RightParenthesis,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RighBrace,
            "," => TokenType::Comma,
            ";" => TokenType::Semicolon,
            "" => TokenType::EOF,
            _ => TokenType::Illegal,
        }
    }
}

struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    character: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            character: char::default(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read_position >= self.input.len() {
            self.character = char::from_u32(0).unwrap();
        } else {
            self.character = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
        Some(Token {
            token_type: self.character.into(),
            literal: self.character,
        })
    }
}
fn test() {
    let input = "=+(){},;";

    let expected_output = vec![
        Token {
            token_type: TokenType::Assign,
            literal: "=".chars().next().unwrap(),
        },
        Token {
            token_type: TokenType::Plus,
            literal: "+".chars().next().unwrap(),
        },
        Token {
            token_type: TokenType::LeftParenthesis,
            literal: "(".chars().next().unwrap(),
        },
        Token {
            token_type: TokenType::RightParenthesis,
            literal: ")".chars().next().unwrap(),
        },
        Token {
            token_type: TokenType::LeftBrace,
            literal: "}".chars().next().unwrap(),
        },
        Token {
            token_type: TokenType::RighBrace,
            literal: "}".chars().next().unwrap(),
        },
        Token {
            token_type: TokenType::Comma,
            literal: ",".chars().next().unwrap(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".chars().next().unwrap(),
        },
        Token {
            token_type: TokenType::EOF,
            literal: "",
        },
    ];

    let lexer = Lexer::new(input);

    for (index, token) in lexer.enumerate() {
        assert_eq!(token.token_type, expected_output[index].token_type);
        assert_eq!(token.literal, expected_output[index].literal);
    }
}
