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

    pub fn generate(input: &str) -> Result<Vec<Token>, String> {
        parse(&mut input.chars().peekable())
    }

    fn parse(iter: &mut Peekable<Chars>) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(&c) = iter.peek() {
            if c.is_whitespace() {
                iter.next();
                continue;
            }
            let token = match c {
                '{' | '}' | '[' | ']' | ':' | ',' => parse_simple_token(iter)?,
                '"' => parse_string(iter)?,
                '0'..='9' => parse_number(iter)?,
                'a'..='z' | 'A'..='Z' => parse_keyword(iter)?,
                _ => return Err(format!("Unexpected character: '{}'", c)),
            };
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn parse_simple_token(iter: &mut Peekable<Chars>) -> Result<Token, String> {
        let character = iter.next().unwrap(); // consume the character
        let token_type = match character {
            '{' => TokenType::OpenObject,
            '}' => TokenType::CloseObject,
            '[' => TokenType::OpenArray,
            ']' => TokenType::CloseArray,
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            _ => return Err(format!("Unexpected simple token: '{}'", character)),
        };
        Ok(Token {
            token_type,
            value: character.to_string(),
        })
    }

    fn parse_string(iter: &mut Peekable<Chars>) -> Result<Token, String> {
        consume_char(iter, '"')?; // consume opening quote
        let string: String = iter.peeking_take_while(|&c| c != '"').collect();
        consume_char(iter, '"')?; // consume closing quote
        Ok(Token {
            token_type: TokenType::String,
            value: string,
        })
    }

    fn parse_number(iter: &mut Peekable<Chars>) -> Result<Token, String> {
        let number_str: String = iter
            .peeking_take_while(|c| c.is_digit(10) || *c == '.')
            .collect();
        Ok(Token {
            token_type: TokenType::Number,
            value: number_str,
        })
    }

    fn parse_keyword(iter: &mut Peekable<Chars>) -> Result<Token, String> {
        let keyword: String = iter.peeking_take_while(|c| c.is_alphabetic()).collect();
        let token_type = match keyword.as_str() {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => return Err(format!("Unexpected keyword: '{}'", keyword)),
        };
        Ok(Token {
            token_type,
            value: keyword,
        })
    }

    fn consume_char(iter: &mut Peekable<Chars>, expected: char) -> Result<char, String> {
        match iter.next() {
            Some(c) if c == expected => Ok(c),
            Some(c) => Err(format!("Expected '{}', but found '{}'", expected, c)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ASTNode {
    Object(AstObjectNode),
    Array(AstArrayNode),
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

pub type AstObjectNode = Vec<(String, ASTNode)>;

pub type AstArrayNode = Vec<ASTNode>;

pub mod parser {
    use super::{ASTNode, AstArrayNode, AstObjectNode, Token, TokenType};
    use std::iter::Peekable;
    use std::slice::Iter;

    pub fn generate(tokens: &[Token]) -> Result<ASTNode, String> {
        parse(&mut tokens.iter().peekable())
    }

    fn parse(iter: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
        let token = iter.peek().ok_or("Unexpected end of input")?;
        match token.token_type {
            TokenType::OpenObject => Ok(ASTNode::Object(parse_object(iter)?)),
            TokenType::OpenArray => Ok(ASTNode::Array(parse_array(iter)?)),
            TokenType::True
            | TokenType::False
            | TokenType::Null
            | TokenType::Number
            | TokenType::String => parse_basic(iter),
            _ => Err("Invalid JSON token".to_string()),
        }
    }

    fn parse_basic(iter: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
        let token = iter.next().ok_or("Unexpected end of input")?;
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

    fn parse_object(iter: &mut Peekable<Iter<Token>>) -> Result<AstObjectNode, String> {
        consume_token(iter, TokenType::OpenObject)?;
        let mut properties = Vec::new();
        while let Some(token) = iter.peek() {
            if token.token_type == TokenType::CloseObject {
                break;
            }
            // resolve "key": value
            let key = consume_string(iter)?;
            consume_token(iter, TokenType::Colon)?;
            let value = parse(iter)?;
            properties.push((key, value));

            // check separator
            match iter.peek().map(|t| t.token_type) {
                Some(TokenType::Comma) => {
                    iter.next(); // consume comma
                    // check for trailing comma
                    if iter.peek().map(|t| t.token_type) == Some(TokenType::CloseObject) {
                        return Err("Trailing comma in object".to_string());
                    }
                }
                Some(TokenType::CloseObject) => break,
                _ => return Err("Expected ',' or '}' in object".to_string()),
            }
        }
        consume_token(iter, TokenType::CloseObject)?;
        Ok(properties)
    }

    fn parse_array(iter: &mut Peekable<Iter<Token>>) -> Result<AstArrayNode, String> {
        consume_token(iter, TokenType::OpenArray)?;
        let mut elements = Vec::new();

        while let Some(token) = iter.peek() {
            if token.token_type == TokenType::CloseArray {
                break;
            }
            let element = parse(iter)?;
            elements.push(element);
            // handle separator
            match iter.peek().map(|t| t.token_type) {
                Some(TokenType::Comma) => {
                    iter.next(); // consume comma
                    // 检查逗号后面是否紧跟结束符（尾随逗号错误）
                    if iter.peek().map(|t| t.token_type) == Some(TokenType::CloseArray) {
                        return Err("Trailing comma in array".to_string());
                    }
                }
                Some(TokenType::CloseArray) => break, // end of array parsing
                _ => return Err("Expected ',' or ']' in array".to_string()),
            }
        }
        consume_token(iter, TokenType::CloseArray)?;
        Ok(elements)
    }

    fn consume_string(iter: &mut Peekable<Iter<Token>>) -> Result<String, String> {
        match iter.next() {
            Some(token) if token.token_type == TokenType::String => Ok(token.value.clone()),
            Some(_) => Err("Expected string".to_string()),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn consume_token(iter: &mut Peekable<Iter<Token>>, expected: TokenType) -> Result<(), String> {
        match iter.next() {
            Some(token) if token.token_type == expected => Ok(()),
            Some(_) => Err(format!("Expected {:?}, found unexpected token", expected)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}
