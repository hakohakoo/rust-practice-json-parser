
const OPEN_OBJECT: &str = "{";
const CLOSE_OBJECT: &str = "}";
const OPEN_ARRAY: &str = "[";
const CLOSE_ARRAY: &str = "]";
const STRING: &str = "string";
const NUMBER: &str = "number";
const TRUE: &str = "true";
const FALSE: &str = "false";
const NULL: &str = "null";
const COLON: &str = ":";
const COMMA: &str = ",";

#[derive(Debug)]
enum TokenType {
    OpenObject,
    CloseObject,
    OpenArray,
    CloseArray,
    String,
    Number,
    True,
    False,
    Null,
    Colon,
    Comma,
}

fn main() {
    let mut lex1 = Lexer::new(r#"{"aa1  23wd2": 123, "b": [true, false, null], "c": { "c1": 100, "c2": false }"}"#);
    lex1.generate();
    println!("{:#?}", lex1.token_list);
}

#[derive(Debug)]
struct Lexer {
    input: String,
    position: usize,
    token_list: Vec<Token>,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.to_string(),
            position: 0,
            token_list: Vec::new(),
        }
    }

    fn generate(&mut self) {
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            match current_char {
                ' ' | '\n' | '\t' | '\r' => {
                    self.position += 1; // Skip whitespace
                }
                '{' => {
                    self.token_list.push(Token {
                        token_type: TokenType::OpenObject,
                        value: OPEN_OBJECT.to_string(),
                    });
                    self.position += 1;
                }
                '}' => {
                    self.token_list.push(Token {
                        token_type: TokenType::CloseObject,
                        value: CLOSE_OBJECT.to_string(),
                    });
                    self.position += 1;
                }
                '[' => {
                    self.token_list.push(Token {
                        token_type: TokenType::OpenArray,
                        value: OPEN_ARRAY.to_string(),
                    });
                    self.position += 1;
                }
                ']' => {
                    self.token_list.push(Token {
                        token_type: TokenType::CloseArray,
                        value: CLOSE_ARRAY.to_string(),
                    });
                    self.position += 1;
                }
                ':' => {
                    self.token_list.push(Token {
                        token_type: TokenType::Colon,
                        value: COLON.to_string(),
                    });
                    self.position += 1;
                }
                ',' => {
                    self.token_list.push(Token {
                        token_type: TokenType::Comma,
                        value: COMMA.to_string(),
                    });
                    self.position += 1;
                }
                '"' => {
                    let token = self.generate_string();
                    self.token_list.push(token);
                }
                '0'..='9' => {
                    let token = self.generate_number();
                    self.token_list.push(token);
                }
                _ => {
                    let token = self.generate_keyword();
                    self.token_list.push(token);
                }
            }
        }
        self.position = 0; 
    }

    fn generate_string(&mut self) -> Token {
        self.position += 1; // Skip opening quote
        let start = self.position;
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if current_char == '"' {
                break;
            }
            self.position += 1;
        }
        self.position += 1; // Skip closing quote
        return Token {
            token_type: TokenType::String,
            value: self.input[start..self.position - 1].to_string(),
        };
    }

    fn generate_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if !current_char.is_digit(10) {
                break;
            }
            self.position += 1;
        }
        let number_str = &self.input[start..self.position];
        return Token {
            token_type: TokenType::Number,
            value: number_str.to_string(),
        };
    }

    fn generate_keyword(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if !current_char.is_alphabetic() {
                break;
            }
            self.position += 1;
        }
        let keyword_str = &self.input[start..self.position];
        let token_type = match keyword_str {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => panic!("Unknown keyword: {}", keyword_str),
        };
        return Token {
            token_type,
            value: keyword_str.to_string(),
        };
    }
}

#[derive(Debug)]
struct ASTNode {
    trees: Vec<ASTNode>,
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    position: usize,
    ast: Option<ASTNode>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
            ast: None,
        }
    }

    fn parse(&mut self) {
        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            match token.token_type {
                TokenType::String => {
                    // Handle string value
                }
                TokenType::Number => {
                    // Handle number value
                }
                TokenType::True | TokenType::False | TokenType::Null => {
                    // Handle literal values
                }
                _ => {}
            }
            self.position += 1;
        }
    }
}