mod lexer {
    use itertools::Itertools;
    use std::iter::Peekable;
    use std::str::Chars;

    #[derive(Debug, PartialEq, Copy, Clone)]
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

    #[derive(Debug)]
    struct Token {
        token_type: TokenType,
        value: String,
    }

    #[derive(Debug)]
    struct Lexer {
        input: String,
        token_list: Vec<Token>,
    }

    impl Lexer {
        fn new(input: &str) -> Self {
            Lexer {
                input: input.to_string(),
                token_list: Vec::new(),
            }
        }

        fn generate(&mut self) {
            let mut iter = self.input.chars().peekable();

            while let Some(current_char) = iter.next() {
                let token = match current_char {
                    ' ' | '\n' | '\t' | '\r' => continue,
                    '{' | '}' | '[' | ']' | ':' | ',' => Self::simple_token(current_char),
                    '"' => Self::parse_string(&mut iter),
                    '0'..='9' => Self::parse_number(current_char, &mut iter),
                    'a'..='z' | 'A'..='Z' => Self::parse_keyword(current_char, &mut iter),
                    _ => panic!("Unexpected character: '{}'", current_char),
                };
                self.token_list.push(token);
            }
        }

        fn simple_token(ch: char) -> Token {
            use TokenType::*;
            let token_type = match ch {
                '{' => OpenObject,
                '}' => CloseObject,
                '[' => OpenArray,
                ']' => CloseArray,
                ':' => Colon,
                ',' => Comma,
                _ => unreachable!(),
            };
            Token {
                token_type,
                value: ch.to_string(),
            }
        }

        fn parse_string(iter: &mut Peekable<Chars<'_>>) -> Token {
            let value: String = iter.peeking_take_while(|&c| c != '"').collect();
            iter.next(); // consume closing quote
            Token {
                token_type: TokenType::String,
                value,
            }
        }

        fn parse_number(first_digit: char, iter: &mut Peekable<Chars<'_>>) -> Token {
            let number: String = std::iter::once(first_digit)
                .chain(iter.peeking_take_while(|c| c.is_ascii_digit()))
                .collect();
            Token {
                token_type: TokenType::Number,
                value: number,
            }
        }

        fn parse_keyword(first_char: char, iter: &mut Peekable<Chars<'_>>) -> Token {
            let keyword: String = std::iter::once(first_char)
                .chain(iter.peeking_take_while(|c| c.is_alphabetic()))
                .collect();

            let token_type = match keyword.as_str() {
                "true" => TokenType::True,
                "false" => TokenType::False,
                "null" => TokenType::Null,
                _ => panic!("Unknown keyword: {}", keyword),
            };

            Token {
                token_type,
                value: keyword,
            }
        }
    }
}
