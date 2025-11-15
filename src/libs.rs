#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
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

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub mod lexer {
    use super::{Token, TokenType};
    use itertools::Itertools;
    use std::iter::Peekable;
    use std::str::Chars;

    type LexResult<T> = Result<T, String>;

    #[derive(Debug)]
    pub struct Lexer<'a> {
        pub input: Peekable<Chars<'a>>,
        pub token_list: Option<Vec<Token>>,
    }

    impl<'a> Lexer<'a> {
        pub fn new(input: &'a str) -> Self {
            Lexer {
                input: input.chars().peekable(),
                token_list: None,
            }
        }

        pub fn generate(&mut self) -> LexResult<()> {
            self.token_list = Some(self.parse()?);
            Ok(())
        }

        pub fn parse(&mut self) -> LexResult<Vec<Token>> {
            let mut token_list = Vec::new();
            while let Some(&current_char) = self.input.peek() {
                match current_char {
                    ' ' | '\n' | '\t' | '\r' => {
                        self.input.next();
                        continue;
                    }
                    '{' | '}' | '[' | ']' | ':' | ',' => {
                        let token = self.parse_simple_token();
                        token_list.push(token);
                    }
                    '"' => {
                        let token = self.parse_string()?;
                        token_list.push(token);
                    }
                    '0'..='9' => {
                        let token = self.parse_number();
                        token_list.push(token);
                    }
                    'a'..='z' | 'A'..='Z' => {
                        let token = self.parse_keyword()?;
                        token_list.push(token);
                    }
                    _ => return Err(format!("Unexpected character: '{}'", current_char)),
                }
            }
            Ok(token_list)
        }

        fn parse_simple_token(&mut self) -> Token {
            let ch = self.input.next().unwrap(); // consume the character
            Token {
                token_type: match ch {
                    '{' => TokenType::OpenObject,
                    '}' => TokenType::CloseObject,
                    '[' => TokenType::OpenArray,
                    ']' => TokenType::CloseArray,
                    ':' => TokenType::Colon,
                    ',' => TokenType::Comma,
                    _ => unreachable!(),
                },
                value: ch.to_string(),
            }
        }

        fn parse_string(&mut self) -> LexResult<Token> {
            self.consume_char('"')?; // consume opening quote
            let value: String = self.input.peeking_take_while(|&c| c != '"').collect();
            self.consume_char('"')?; // consume closing quote
            Ok(Token {
                token_type: TokenType::String,
                value,
            })
        }

        fn parse_number(&mut self) -> Token {
            Token {
                token_type: TokenType::Number,
                value: self
                    .input
                    .peeking_take_while(|c| c.is_ascii_digit())
                    .collect(),
            }
        }

        fn parse_keyword(&mut self) -> LexResult<Token> {
            let keyword: String = self
                .input
                .peeking_take_while(|c| c.is_alphabetic())
                .collect();
            Ok(Token {
                token_type: match keyword.as_str() {
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    "null" => TokenType::Null,
                    _ => return Err(format!("Unknown keyword: {}", keyword)),
                },
                value: keyword,
            })
        }

        fn consume_char(&mut self, expected: char) -> LexResult<char> {
            match self.input.next() {
                Some(c) if c == expected => Ok(c),
                Some(c) => Err(format!("Expected '{}', but found '{}'", expected, c)),
                None => return Err("Unexpected end of input".to_string()),
            }
        }
    }
}

#[derive(Debug)]
enum ASTNode {
    Object(AstObjectNode),
    Array(AstArrayNode),
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

type AstObjectNode = Vec<(String, ASTNode)>;

type AstArrayNode = Vec<ASTNode>;

pub mod parser {
    use super::{ASTNode, AstArrayNode, AstObjectNode, Token, TokenType};
    use std::iter::Peekable;
    use std::slice::Iter;

    type ParseResult<T> = Result<T, String>;

    #[derive(Debug)]
    pub struct Parser<'a> {
        tokens: Peekable<Iter<'a, Token>>,
        ast: Option<ASTNode>,
    }

    impl<'a> Parser<'a> {
        pub fn new(tokens: &'a [Token]) -> Self {
            Parser {
                tokens: tokens.iter().peekable(),
                ast: None,
            }
        }

        pub fn generate(&mut self) -> ParseResult<()> {
            self.ast = Some(self.parse()?);
            Ok(())
        }

        fn parse(&mut self) -> ParseResult<ASTNode> {
            let token = self.tokens.peek().ok_or("Unexpected end of input")?;
            match token.token_type {
                TokenType::OpenObject => Ok(ASTNode::Object(self.parse_object()?)),
                TokenType::OpenArray => Ok(ASTNode::Array(self.parse_array()?)),
                TokenType::True
                | TokenType::False
                | TokenType::Null
                | TokenType::Number
                | TokenType::String => self.parse_basic(),
                _ => Err("Invalid JSON token".to_string()),
            }
        }

        fn parse_basic(&mut self) -> ParseResult<ASTNode> {
            let token = self.tokens.next().ok_or("Unexpected end of input")?;
            match token.token_type {
                TokenType::True => Ok(ASTNode::True),
                TokenType::False => Ok(ASTNode::False),
                TokenType::Null => Ok(ASTNode::Null),
                TokenType::Number => {
                    let number = token.value.parse::<f64>().map_err(|_| "Invalid number")?;
                    Ok(ASTNode::Number(number))
                }
                TokenType::String => Ok(ASTNode::String(token.value.clone())),
                _ => Err("Invalid token".to_string()),
            }
        }

        fn parse_object(&mut self) -> ParseResult<AstObjectNode> {
            self.consume_token(TokenType::OpenObject)?;

            let mut properties = Vec::new();

            if !matches!(self.tokens.peek(), Some(t) if t.token_type == TokenType::CloseObject) {
                while !matches!(self.tokens.peek(), Some(t) if t.token_type == TokenType::CloseObject)
                {
                    // 解析 "key": value
                    let key = self.consume_string()?;
                    self.consume_token(TokenType::Colon)?;
                    let value = self.parse()?;
                    properties.push((key, value));

                    // 检查分隔符
                    match self.tokens.peek().map(|t| t.token_type) {
                        Some(TokenType::Comma) => {
                            self.tokens.next();
                        }
                        Some(TokenType::CloseObject) => break,
                        _ => return Err("Expected ',' or '}' in object".to_string()),
                    }
                }
            }

            self.consume_token(TokenType::CloseObject)?;
            Ok(properties)
        }

        fn parse_array(&mut self) -> ParseResult<AstArrayNode> {
            self.consume_token(TokenType::OpenArray)?;

            // 处理空数组
            if self
                .tokens
                .peek()
                .map(|token| token.token_type == TokenType::CloseArray)
                .unwrap_or(false)
            {
                self.consume_token(TokenType::CloseArray)?;
                return Ok(Vec::new());
            }

            let mut elements = Vec::new();

            loop {
                // 解析数组元素
                let element = self.parse()?;
                elements.push(element);

                // 处理分隔符（内联handle_separator的逻辑）
                let token = self.tokens.peek().ok_or("Unexpected end of input")?;
                match token.token_type {
                    TokenType::Comma => {
                        self.tokens.next(); // 消费逗号
                        continue; // 继续解析下一个元素
                    }
                    TokenType::CloseArray => break, // 结束数组解析
                    _ => return Err("Invalid separator".to_string()),
                }
            }

            self.consume_token(TokenType::CloseArray)?;
            Ok(elements)
        }

        fn consume_string(&mut self) -> ParseResult<String> {
            match self.tokens.next() {
                Some(token) if token.token_type == TokenType::String => Ok(token.value.clone()),
                Some(_) => Err("Expected string".to_string()),
                None => Err("Unexpected end of input".to_string()),
            }
        }

        fn consume_token(&mut self, expected: TokenType) -> ParseResult<()> {
            match self.tokens.next() {
                Some(token) if token.token_type == expected => Ok(()),
                Some(_) => Err(format!("Expected {:?}, found unexpected token", expected)),
                None => Err("Unexpected end of input".to_string()),
            }
        }
    }
}
