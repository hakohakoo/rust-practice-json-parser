## 1. 基础概念

### 1.1 什么是 Lexer（词法分析器）？

**Lexer** 是编译器的第一阶段，负责将原始的字符流转换为有意义的词法单元（Token）。它需要读取输入字符串，识别出各种符号、关键字、数字等，然后将识别出的字符序列分类为不同类型的 Token，同时过滤掉空白字符、注释等无关内容。

例如，对于 JSON 字符串 `{"name": "John"}`:

-   `{` → OpenObject Token
-   `"name"` → String Token (value: "name")
-   `:` → Colon Token
-   `"John"` → String Token (value: "John")
-   `}` → CloseObject Token

### 1.2 什么是 Token 流？

**Token 流** 是 Lexer 的输出，Parser 的输入。它是一个 Token 序列，每个 Token 包含类型信息（这是什么类型的 Token，如字符串、数字、符号等）和值信息（Token 的具体内容）。

### 1.3 什么是 Parser（语法分析器）？

**Parser** 接收 Token 流，根据语法规则构建抽象语法树（AST）。它需要检查 Token 序列是否符合 JSON 语法规则，识别出对象、数组、键值对等结构关系，并将扁平的 Token 流转换为层次化的 AST。

### 1.4 完整流程示例

**输入 JSON**：`{"age": 25}`

**Lexer 阶段**：

```
字符流: { " a g e " : 2 5 }
   ↓
Token流: [OpenObject, String("age"), Colon, Number("25"), CloseObject]
```

**Parser 阶段**：

```
Token流: [OpenObject, String("age"), Colon, Number("25"), CloseObject]
   ↓
AST树: Object([("age", Number(25.0))])
```

## 2. 项目概览

本项目实现了一个简陋的 RUST 版的 JSON 解析流：

```
JSON 字符串 → Lexer → Token 流 → Parser → AST 树
```

## 3. 核心组件

### 3.1 Token 定义

首先，需要分析 JSON 格式包含哪些基本元素。JSON 语法包含结构符号（`{` `}` `[` `]`）用于定义对象和数组的边界，分隔符号（`:` `,`）用于分离键值对和元素，以及各种数据类型如字符串、数字、布尔值、null，还有三个特殊的字面量关键字 `true` `false` `null`。

基于上述分析，设计了对应的 Token 类型：

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    OpenObject, CloseObject,    // { } - 对象边界
    OpenArray, CloseArray,      // [ ] - 数组边界
    String, Number,             // "text", 123 - 数据类型
    True, False, Null,          // 布尔值和空值字面量
    Colon, Comma,              // : , - 分隔符
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,  // Token 的类型分类
    pub value: String,          // Token 的原始字符串值
}
```

**枚举的使用：** Rust 的枚举确保我们只能处理预定义的 Token 类型，提供了类型安全保障，同时可以使用 `match` 语句优雅地处理不同的 Token。

**Copy 宏的使用：** 这样做可以避免不必要的内存分配和移动，提升性能，同时使用起来更加便利，可以轻松复制和比较 Token 类型。

**Token 结构体中 value 字段的作用：** 因为字符串和数字需要保留具体的值信息，同时这也便于错误报告和调试输出。

### 3.2 Lexer

#### a. 主入口函数 `generate`

```rust
pub fn generate(input: &str) -> Result<Vec<Token>, String> {
    parse(&mut input.chars().peekable())
}
```

这是 Lexer 的对外接口，接收字符串切片，创建字符迭代器的 Peekable 包装后调用内部解析函数。

#### b. 核心解析函数 `parse`

```rust
fn parse(iter: &mut Peekable<Chars>) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    while let Some(&c) = iter.peek() {
        if c.is_whitespace() {
            iter.next(); // 跳过空白字符
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
```

这是词法分析的核心函数。通过 `peek()` 查看当前字符而不消费它，根据字符类型决定调用哪个专门的解析函数。使用了多重匹配和范围匹配来处理不同的字符类型。

#### c. 简单符号解析函数 `parse_simple_token`

```rust
fn parse_simple_token(iter: &mut Peekable<Chars>) -> Result<Token, String> {
    let character = iter.next().unwrap(); // 消费字符
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
```

处理 JSON 中的单字符符号。消费一个字符后根据字符类型创建对应的 Token。使用 `unreachable!()` 表示不可能到达的分支。

#### d. 字符串解析函数 `parse_string`

```rust
fn parse_string(iter: &mut Peekable<Chars>) -> Result<Token, String> {
    consume_char(iter, '"')?; // 消费开始引号
    let string: String = iter.peeking_take_while(|&c| c != '"').collect();
    consume_char(iter, '"')?; // 消费结束引号
    Ok(Token {
        token_type: TokenType::String,
        value: string,
    })
}
```

处理 JSON 字符串的解析。消费开始和结束的双引号，使用 `peeking_take_while` 收集引号之间的所有字符内容。

#### e. 数字解析函数 `parse_number`

```rust
fn parse_number(iter: &mut Peekable<Chars>) -> Result<Token, String> {
    let number_str: String = iter
        .peeking_take_while(|c| c.is_digit(10) || *c == '.')
        .collect();
    Ok(Token {
        token_type: TokenType::Number,
        value: number_str,
    })
}
```

处理 JSON 数字的解析。收集连续的数字字符和小数点，构成完整的数字字符串。支持整数和浮点数格式。

#### f. 关键字解析函数 `parse_keyword`

```rust
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
```

处理 JSON 关键字（true、false、null）的解析。收集连续的字母字符，然后匹配已知的关键字，如果不匹配则返回错误。

### 3.3 AST 定义

JSON 支持六种数据类型：对象（键值对的无序集合，如 `{"key": value}`）、数组（值的有序列表，如 `[value1, value2]`）、字符串（双引号包围的文本，如 `"hello"`）、数字（整数或浮点数，如 `42`, `3.14`）、布尔值（`true` 或 `false`）以及空值（`null`）。

基于上述分析，设计了对应的 AST 数据结构：

```rust
#[derive(Debug)]
#[allow(dead_code)]
pub enum ASTNode {
    Object(AstObjectNode),    // 对象节点
    Array(AstArrayNode),      // 数组节点
    String(String),           // 字符串节点
    Number(f64),              // 数字节点（统一用 f64）
    True, False, Null,        // 字面量节点
}

// 类型别名
pub type AstObjectNode = Vec<(String, ASTNode)>;
pub type AstArrayNode = Vec<ASTNode>;
```

### 3.4 Parser

#### a. 主入口函数 `generate`

```rust
pub fn generate(tokens: &[Token]) -> Result<ASTNode, String> {
    parse(&mut tokens.iter().peekable())
}
```

接收 Token 切片，创建 Peekable 迭代器后调用内部解析函数。

#### b. 解析函数 `parse`

```rust
fn parse(iter: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
    let token = iter.peek().ok_or("Unexpected end of input")?;
    match token.token_type {
        TokenType::OpenObject => Ok(ASTNode::Object(parse_object(iter)?)),
        TokenType::OpenArray => Ok(ASTNode::Array(parse_array(iter)?)),
        TokenType::True | TokenType::False | TokenType::Null |
        TokenType::Number | TokenType::String => parse_basic(iter),
        _ => Err("Invalid JSON token".to_string()),
    }
}
```

这是递归下降解析的核心调度器。通过 `peek()` 查看下一个 Token 而不消费它，根据 Token 类型决定调用哪个专门的解析函数。

#### c. 对象解析函数 `parse_object`

```rust
fn parse_object(iter: &mut Peekable<Iter<Token>>) -> Result<AstObjectNode, String> {
    consume_token(iter, TokenType::OpenObject)?;  // 消费开始 '{'
    let mut properties = Vec::new();

    while let Some(token) = iter.peek() {
        if token.token_type == TokenType::CloseObject {
            break;  // 遇到 '}' 提前结束（处理空对象）
        }

        // 解析 "key": value 模式
        let key = consume_string(iter)?;           // 必须是字符串键
        consume_token(iter, TokenType::Colon)?;   // 必须有冒号分隔符
        let value = parse(iter)?;                 // 递归解析值（可能是任何 JSON 类型）
        properties.push((key, value));

        // 处理键值对之间的分隔符
        match iter.peek().map(|t| t.token_type) {
            Some(TokenType::Comma) => {
                iter.next(); // 消费逗号
                // 检查尾随逗号
                if iter.peek().map(|t| t.token_type) == Some(TokenType::CloseObject) {
                    return Err("Trailing comma in object".to_string());
                }
            }
            Some(TokenType::CloseObject) => break,          // 遇到结束符，准备退出循环
            _ => return Err("Expected ',' or '}' in object".to_string()),
        }
    }

    consume_token(iter, TokenType::CloseObject)?;  // 消费结束 '}'
    Ok(properties)
}
```

处理 JSON 对象的解析。先消费开始大括号，然后循环解析键值对。每个键值对包含字符串键、冒号分隔符和任意类型的值（通过递归调用 `parse`）。

#### d. 数组解析函数 `parse_array`

```rust
fn parse_array(iter: &mut Peekable<Iter<Token>>) -> Result<AstArrayNode, String> {
    consume_token(iter, TokenType::OpenArray)?;   // 消费开始 '['
    let mut elements = Vec::new();

    while let Some(token) = iter.peek() {
        if token.token_type == TokenType::CloseArray {
            break;  // 遇到 ']' 提前结束（处理空数组）
        }

        let element = parse(iter)?;  // 递归解析元素（可能是任何 JSON 类型）
        elements.push(element);

        // 处理元素之间的分隔符
        match iter.peek().map(|t| t.token_type) {
            Some(TokenType::Comma) => {
                iter.next(); // 消费逗号
                // 检查尾随逗号
                if iter.peek().map(|t| t.token_type) == Some(TokenType::CloseArray) {
                    return Err("Trailing comma in array".to_string());
                }
            }
            Some(TokenType::CloseArray) => break,          // 遇到结束符，准备退出循环
            _ => return Err("Expected ',' or ']' in array".to_string()),
        }
    }

    consume_token(iter, TokenType::CloseArray)?;  // 消费结束 ']'
    Ok(elements)
}
```

处理 JSON 数组的解析。结构与对象解析类似，但更简单：只需要递归解析元素值，不需要处理键名和冒号。同样使用分隔符来判断是否继续解析下一个元素。

#### e. 基本类型解析函数 `parse_basic`

```rust
fn parse_basic(iter: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
    let token = iter.next().ok_or("Unexpected end of input")?;  // 消费 Token
    match token.token_type {
        TokenType::True => Ok(ASTNode::True),
        TokenType::False => Ok(ASTNode::False),
        TokenType::Null => Ok(ASTNode::Null),
        TokenType::Number => {
            let number = token.value.parse::<f64>().map_err(|_| "Invalid number")?;
            Ok(ASTNode::Number(number))  // 字符串转数字，可能失败
        }
        TokenType::String => Ok(ASTNode::String(token.value.clone())),
        _ => Err("Invalid token".to_string()),
    }
}
```

处理 JSON 的叶子节点（基本数据类型）。注意这里使用 `next()` 而不是 `peek()`，因为需要消费 Token。对于数字类型需要进行字符串到浮点数的转换，可能产生解析错误。

## 4. 运行和测试

```bash
cargo run
```
