// mod lexer {
//     use itertools::Itertools;

//     #[derive(Debug, PartialEq)]
//     enum TokenType {
//         OpenObject,
//         CloseObject,
//         OpenArray,
//         CloseArray,
//         String,
//         Number,
//         True,
//         False,
//         Null,
//         Colon,
//         Comma,
//     }

//     #[derive(Debug)]
//     struct Lexer {
//         input: String,
//         token_list: Vec<Token>,
//     }

//     #[derive(Debug)]
//     struct Token {
//         token_type: TokenType,
//         value: String,
//     }

//     impl Lexer {
//         fn new(input: &str) -> Self {
//             Lexer {
//                 input: input.to_string(),
//                 token_list: Vec::new(),
//             }
//         }

//         fn generate(&mut self) {
//             let mut iter = self.input.chars().peekable();

//             while let Some(current_char) = iter.next() {
//                 match current_char {
//                     ' ' | '\n' | '\t' | '\r' => continue, // Skip whitespace
//                     '{' => self.token_list.push(Token {
//                         token_type: TokenType::OpenObject,
//                         value: '{'.to_string(),
//                     }),
//                     '}' => self.token_list.push(Token {
//                         token_type: TokenType::CloseObject,
//                         value: '}'.to_string(),
//                     }),
//                     '[' => self.token_list.push(Token {
//                         token_type: TokenType::OpenArray,
//                         value: '['.to_string(),
//                     }),
//                     ']' => self.token_list.push(Token {
//                         token_type: TokenType::CloseArray,
//                         value: ']'.to_string(),
//                     }),
//                     ':' => self.token_list.push(Token {
//                         token_type: TokenType::Colon,
//                         value: ':'.to_string(),
//                     }),
//                     ',' => self.token_list.push(Token {
//                         token_type: TokenType::Comma,
//                         value: ','.to_string(),
//                     }),
//                     '"' => {
//                         // 0. 单独抽出函数，基于 position 记录
//                         // 1. 单独抽出函数，然后 while let 遍历
//                         // 2. 直接在这里用 take_while，但是需要单独获取 &mut
//                         // let local_iter = &mut iter;
//                         // 3. 使用 by_ref() 方法创建一个可变借用
//                         self.token_list.push(Token {
//                             token_type: TokenType::String,
//                             value: iter.by_ref().take_while(|&c| c != '"').collect(),
//                         });
//                     }
//                     '0'..='9' => {
//                         self.token_list.push(Token {
//                             token_type: TokenType::Number,
//                             value: current_char.to_string()
//                                 + &iter
//                                     .by_ref()
//                                     .peeking_take_while(|&c| c.is_digit(10))
//                                     .collect::<String>(),
//                         });
//                     }
//                     _ => {
//                         let keyword = current_char.to_string()
//                             + &iter
//                                 .by_ref()
//                                 .peeking_take_while(|&c| c.is_alphabetic())
//                                 .collect::<String>();
//                         let token_type = match keyword.as_str() {
//                             "true" => TokenType::True,
//                             "false" => TokenType::False,
//                             "null" => TokenType::Null,
//                             _ => panic!("Unknown keyword: {}", keyword),
//                         };
//                         self.token_list.push(Token {
//                             token_type,
//                             value: keyword,
//                         });
//                     }
//                 }
//             }
//         }

//         // fn generate_string(iter: &mut Chars<'_>) -> Token {
//         //     let mut string_value = String::new();
//         //     while let Some(current_char) = iter.next() {
//         //         if current_char == '"' {
//         //             break; // Closing quote found
//         //         }
//         //         string_value.push(current_char);
//         //     }
//         //     return Token {
//         //         token_type: TokenType::String,
//         //         value: string_value,
//         //     };
//         // }

//         // fn generate_number(&mut self) -> Token {
//         //     let start = self.position;
//         //     while self.position < self.input.len() {
//         //         let current_char = self.input.chars().nth(self.position).unwrap();
//         //         if !current_char.is_digit(10) {
//         //             break;
//         //         }
//         //         self.position += 1;
//         //     }
//         //     let number_str = &self.input[start..self.position];
//         //     return Token {
//         //         token_type: TokenType::Number,
//         //         value: number_str.to_string(),
//         //     };
//         // }

//         // fn generate_keyword(&mut self) -> Token {
//         //     let start = self.position;
//         //     while self.position < self.input.len() {
//         //         let current_char = self.input.chars().nth(self.position).unwrap();
//         //         if !current_char.is_alphabetic() {
//         //             break;
//         //         }
//         //         self.position += 1;
//         //     }
//         //     let keyword_str = &self.input[start..self.position];
//         //     let token_type = match keyword_str {
//         //         "true" => TokenType::True,
//         //         "false" => TokenType::False,
//         //         "null" => TokenType::Null,
//         //         _ => panic!("Unknown keyword: {}", keyword_str),
//         //     };
//         //     return Token {
//         //         token_type,
//         //         value: keyword_str.to_string(),
//         //     };
//         // }
//     }
// }
