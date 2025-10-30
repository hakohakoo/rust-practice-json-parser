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

#[derive(Debug, PartialEq)]
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
    let mut lex1 = Lexer::new(
        r#"{"aa1  23wd2": 123, "b": [true, false, null], "c": { "c1": 100, "c2": false }}"#,
    );
    lex1.generate();
    println!("{:#?}", lex1.token_list);
    let mut parser = Parser::new(lex1.token_list);
    parser.generate();
    println!("{:#?}", parser.ast);
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

type ObjectNode = Vec<(String, ASTNode)>;

type ArrayNode = Vec<ASTNode>;

#[derive(Debug)]
enum ASTNode {
    Object(ObjectNode),
    Array(ArrayNode),
    String(String),
    Number(f64),
    True,
    False,
    Null,
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

    fn generate(&mut self) {
        self.ast = Some(self.parse());
    }

    fn parse(&mut self) -> ASTNode {
        let token = &self.tokens[self.position];
        match token.token_type {
            TokenType::OpenObject => {
                return ASTNode::Object(self.parse_object());
            }
            TokenType::OpenArray => {
                return ASTNode::Array(self.parse_array());
            }
            TokenType::True
            | TokenType::False
            | TokenType::Null
            | TokenType::Number
            | TokenType::String => {
                return self.parse_basic();
            }
            TokenType::CloseArray
            | TokenType::CloseObject
            | TokenType::Colon
            | TokenType::Comma => {
                panic!("Unexpected token: {:?}", token);
            }
        };
    }

    fn parse_basic(&mut self) -> ASTNode {
        let token = &self.tokens[self.position];
        let result = match token.token_type {
            TokenType::True => ASTNode::True,
            TokenType::False => ASTNode::False,
            TokenType::Null => ASTNode::Null,
            TokenType::Number => ASTNode::Number(token.value.parse().unwrap()),
            TokenType::String => ASTNode::String(token.value.clone()),
            _ => panic!("Unexpected token in basic parse: {:?}", token),
        };
        self.position += 1;
        return result;
    }

    fn parse_object(&mut self) -> ObjectNode {
        let mut properties: ObjectNode = vec![];
        self.position += 1; // Skip OPEN_OBJECT
        loop {
            let current_token = &self.tokens[self.position];
            match current_token.token_type {
                TokenType::CloseObject => {
                    break;
                }
                TokenType::String => {
                    let key = current_token.value.clone();
                    self.position += 1; // Move to COLON
                    if self.tokens[self.position].token_type != TokenType::Colon {
                        panic!("Expected COLON after key in object");
                    }
                    self.position += 1; // Move to value
                    let value = self.parse();
                    println!("Value for key {}: {:?}", &key, &value);
                    properties.push((key, value));
                    let next_token = &self.tokens[self.position];
                    println!("Next token after value: {:?}", next_token);
                    if next_token.token_type != TokenType::Comma
                        && next_token.token_type != TokenType::CloseObject
                    {
                        panic!("Expected COMMA or CLOSE_OBJECT after value in object");
                    }
                    if next_token.token_type == TokenType::Comma {
                        self.position += 1; // Move to next key
                    }
                }
                _ => panic!("Unexpected token in object: {:?}", current_token),
            }
        }
        self.position += 1; // Skip CLOSE_OBJECT
        return properties;
    }

    fn parse_array(&mut self) -> ArrayNode {
        let mut elements: ArrayNode = vec![];
        self.position += 1; // Skip OPEN_ARRAY
        loop {
            let current_token = &self.tokens[self.position];
            match current_token.token_type {
                TokenType::CloseArray => {
                    break;
                }
                _ => {
                    let element = self.parse();
                    elements.push(element);
                    let next_token = &self.tokens[self.position];
                    if next_token.token_type != TokenType::Comma
                        && next_token.token_type != TokenType::CloseArray
                    {
                        panic!("Expected COMMA or CLOSE_ARRAY after element in array");
                    }
                    if next_token.token_type == TokenType::Comma {
                        self.position += 1; // Move to next element
                    }
                }
            }
        }
        self.position += 1; // Skip CLOSE_ARRAY
        return elements;
    }
}
