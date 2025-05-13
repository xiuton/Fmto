use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Token {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Equals,
    Dot,
    Include,
    Import,
    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(Token::EOF);
        }

        let c = self.input[self.position];
        self.position += 1;

        match c {
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            ',' => Ok(Token::Comma),
            ':' => Ok(Token::Colon),
            '=' => Ok(Token::Equals),
            '.' => Ok(Token::Dot),
            '"' | '\'' => self.read_string(c),
            '0'..='9' | '-' => self.read_number(c),
            't' | 'f' => self.read_boolean(c),
            'n' => self.read_null(),
            'i' => self.read_include(),
            _ => Err(anyhow!("无效的字符: {}", c)),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let c = self.input[self.position];
            if c.is_whitespace() || c == '#' {
                if c == '#' {
                    while self.position < self.input.len() && self.input[self.position] != '\n' {
                        self.position += 1;
                    }
                } else {
                    self.position += 1;
                }
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self, quote: char) -> Result<Token> {
        let mut result = String::new();
        while self.position < self.input.len() {
            let c = self.input[self.position];
            if c == quote {
                self.position += 1;
                return Ok(Token::String(result));
            }
            result.push(c);
            self.position += 1;
        }
        Err(anyhow!("未闭合的字符串"))
    }

    fn read_number(&mut self, first: char) -> Result<Token> {
        let mut result = String::new();
        result.push(first);
        
        while self.position < self.input.len() {
            let c = self.input[self.position];
            if c.is_digit(10) || c == '.' {
                result.push(c);
                self.position += 1;
            } else {
                break;
            }
        }
        
        match result.parse::<f64>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => Err(anyhow!("无效的数字: {}", result)),
        }
    }

    fn read_boolean(&mut self, first: char) -> Result<Token> {
        let mut result = String::new();
        result.push(first);
        
        while self.position < self.input.len() {
            let c = self.input[self.position];
            if c.is_alphabetic() {
                result.push(c);
                self.position += 1;
            } else {
                break;
            }
        }
        
        match result.as_str() {
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            _ => Err(anyhow!("无效的布尔值: {}", result)),
        }
    }

    fn read_null(&mut self) -> Result<Token> {
        let mut result = String::new();
        result.push('n');
        
        while self.position < self.input.len() {
            let c = self.input[self.position];
            if c.is_alphabetic() {
                result.push(c);
                self.position += 1;
            } else {
                break;
            }
        }
        
        if result == "null" {
            Ok(Token::Null)
        } else {
            Err(anyhow!("无效的 null 值: {}", result))
        }
    }

    fn read_include(&mut self) -> Result<Token> {
        let mut result = String::new();
        result.push('i');
        
        while self.position < self.input.len() {
            let c = self.input[self.position];
            if c.is_alphabetic() {
                result.push(c);
                self.position += 1;
            } else {
                break;
            }
        }
        
        if result == "include" {
            Ok(Token::Include)
        } else {
            Err(anyhow!("无效的 include 关键字: {}", result))
        }
    }
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn new(input: &str) -> Result<Self> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        Ok(Parser {
            lexer,
            current_token,
        })
    }

    fn parse(&mut self) -> Result<Value> {
        let result = self.parse_value()?;
        if !matches!(self.current_token, Token::EOF) {
            return Err(anyhow!("解析未完成"));
        }
        Ok(result)
    }

    fn parse_value(&mut self) -> Result<Value> {
        let token = std::mem::replace(&mut self.current_token, Token::EOF);
        let result = match token {
            Token::String(s) => {
                self.advance()?;
                Value::String(s)
            }
            Token::Number(n) => {
                self.advance()?;
                Value::Number(serde_json::Number::from_f64(n).unwrap())
            }
            Token::Boolean(b) => {
                self.advance()?;
                Value::Bool(b)
            }
            Token::Null => {
                self.advance()?;
                Value::Null
            }
            Token::LeftBrace => self.parse_object()?,
            Token::LeftBracket => self.parse_array()?,
            _ => return Err(anyhow!("无效的值")),
        };
        Ok(result)
    }

    fn parse_object(&mut self) -> Result<Value> {
        let mut map = HashMap::new();
        
        self.advance()?; // 跳过 {
        
        while !matches!(self.current_token, Token::RightBrace) {
            let key = match &self.current_token {
                Token::String(s) => s.clone(),
                _ => return Err(anyhow!("对象键必须是字符串")),
            };
            
            self.advance()?;
            
            match self.current_token {
                Token::Colon | Token::Equals => {
                    self.advance()?;
                }
                _ => return Err(anyhow!("期望 : 或 =")),
            }
            
            let value = self.parse_value()?;
            map.insert(key, value);
            
            if matches!(self.current_token, Token::Comma) {
                self.advance()?;
            }
        }
        
        self.advance()?; // 跳过 }
        Ok(Value::Object(map.into_iter().collect()))
    }

    fn parse_array(&mut self) -> Result<Value> {
        let mut array = Vec::new();
        
        self.advance()?; // 跳过 [
        
        while !matches!(self.current_token, Token::RightBracket) {
            let value = self.parse_value()?;
            array.push(value);
            
            if matches!(self.current_token, Token::Comma) {
                self.advance()?;
            }
        }
        
        self.advance()?; // 跳过 ]
        Ok(Value::Array(array))
    }

    fn advance(&mut self) -> Result<()> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }
}

pub fn parse_hocon(input: &str) -> Result<HashMap<String, Value>> {
    let mut parser = Parser::new(input)?;
    let value = parser.parse()?;
    
    match value {
        Value::Object(map) => Ok(map.into_iter().collect()),
        _ => Err(anyhow!("HOCON 根节点必须是对象")),
    }
} 