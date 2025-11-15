mod libs;

use libs::{lexer, parser};

fn main() {
    println!("=== JSON Parser Testing ===\n");

    // 测试用例
    let test_cases = vec![
        r#"{"name": "John", "age": 30}"#,
        r#"[1, 2, 3, "hello"]"#,
        r#"{"active": true, "data": null}"#,
        r#"{"nested": {"inner": "value"}, "array": [1, 2, 3]}"#,
        r#"[]"#,
        r#"{}"#,
        r#"false"#,
        r#"42"#,
        r#""simple string""#,
    ];

    for (i, json_str) in test_cases.iter().enumerate() {
        println!("--- Test Case {} ---", i + 1);
        println!("Input: {}", json_str);

        test_json_parsing(json_str);
        println!();
    }

    // 测试错误情况
    println!("--- Error Cases ---");
    let error_cases = vec![
        r#"{"name": "John",}"#,   // 多余的逗号
        r#"{"name" "John"}"#,     // 缺少冒号
        r#"{name: "John"}"#,      // 键没有引号
        r#"{"name": undefined}"#, // 未知关键字
    ];

    for (i, json_str) in error_cases.iter().enumerate() {
        println!("Error Case {}: {}", i + 1, json_str);
        test_json_parsing(json_str);
        println!();
    }
}

fn test_json_parsing(input: &str) {
    // 步骤1: 词法分析
    println!("  Step 1: Lexical Analysis");
    match lexer::generate(input) {
        Ok(tokens) => {
            println!("  ✓ Tokens generated successfully:");
            for (i, token) in tokens.iter().enumerate() {
                println!("    {}. {:?}", i + 1, token);
            }

            // 步骤2: 语法分析
            println!("  Step 2: Syntax Analysis");
            match parser::generate(&tokens) {
                Ok(ast) => {
                    println!("  ✓ AST generated successfully:");
                    println!("    {:?}", ast);
                }
                Err(parse_error) => {
                    println!("  ✗ Parse Error: {}", parse_error);
                }
            }
        }
        Err(lex_error) => {
            println!("  ✗ Lexer Error: {}", lex_error);
        }
    }
}

// 演示单独测试 lexer
#[allow(dead_code)]
fn test_lexer_only() {
    let input = r#"{"hello": "world"}"#;
    println!("Testing lexer with: {}", input);

    match lexer::generate(input) {
        Ok(tokens) => {
            println!("Generated {} tokens:", tokens.len());
            for token in tokens {
                println!("  {:?}", token);
            }
        }
        Err(e) => println!("Lexer error: {}", e),
    }
}

// 演示单独测试 parser
#[allow(dead_code)]
fn test_parser_only() {
    use libs::{Token, TokenType};

    // 手动创建一些 tokens 来测试 parser
    let tokens = vec![
        Token {
            token_type: TokenType::OpenObject,
            value: "{".to_string(),
        },
        Token {
            token_type: TokenType::String,
            value: "key".to_string(),
        },
        Token {
            token_type: TokenType::Colon,
            value: ":".to_string(),
        },
        Token {
            token_type: TokenType::String,
            value: "value".to_string(),
        },
        Token {
            token_type: TokenType::CloseObject,
            value: "}".to_string(),
        },
    ];

    println!("Testing parser with manual tokens");
    match parser::generate(&tokens) {
        Ok(ast) => println!("AST: {:?}", ast),
        Err(e) => println!("Parser error: {}", e),
    }
}

// 完整的 JSON 解析流水线
fn parse_json_complete(input: &str) -> Result<libs::ASTNode, String> {
    let tokens = lexer::generate(input)?;
    let ast = parser::generate(&tokens)?;
    Ok(ast)
}

// 演示完整流水线
#[allow(dead_code)]
fn demo_complete_pipeline() {
    let json = r#"{"users": [{"name": "Alice", "age": 25}, {"name": "Bob", "age": 30}]}"#;

    println!("Complete JSON parsing demo:");
    println!("Input: {}", json);

    match parse_json_complete(json) {
        Ok(ast) => {
            println!("Success! Final AST:");
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("Failed: {}", e);
        }
    }
}
