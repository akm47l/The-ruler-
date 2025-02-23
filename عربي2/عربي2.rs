use std::collections::HashMap;

// تعريف أنواع الرموز (Tokens)
#[derive(Debug, PartialEq)]
enum TokenType {
    Identifier,
    Number,
    Keyword,
    Symbol,
    String,
    Comment,
    EndOfFile,
}

// هيكلة الرمز البرمجي (Token)
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
}

// قائمة الكلمات المحجوزة في لغة "عربي"
fn get_keywords() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("عرف".to_string(), TokenType::Keyword);
    keywords.insert("اكتب".to_string(), TokenType::Keyword);
    keywords.insert("ادخل".to_string(), TokenType::Keyword);
    keywords.insert("إذا".to_string(), TokenType::Keyword);
    keywords.insert("كرّر".to_string(), TokenType::Keyword);
    keywords.insert("بينما".to_string(), TokenType::Keyword);
    keywords.insert("نفذ".to_string(), TokenType::Keyword);
    keywords.insert("ثابت".to_string(), TokenType::Keyword);
    keywords
}

// دالة تحليل الرموز (Lexer)
fn lexer(code: &str) -> Vec<Token> {
    let keywords = get_keywords();
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();
    
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c == '#' {  // التعليقات الأحادية
            while let Some(&ch) = chars.peek() {
                if ch != '\n' {
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token { token_type: TokenType::Comment, value: "".to_string() });
        } else if c == '"' || c == '\'' {  // التعامل مع النصوص
            let quote = chars.next().unwrap();
            let mut str = String::new();
            while let Some(&ch) = chars.peek() {
                if ch == quote {
                    chars.next();
                    break;
                } else if ch == '\\' {  // معالجة الرموز الهاربة
                    chars.next();
                    if let Some(escaped) = chars.next() {
                        str.push(escaped);
                    }
                } else {
                    str.push(ch);
                    chars.next();
                }
            }
            tokens.push(Token { token_type: TokenType::String, value: str });
        } else if c.is_alphabetic() {
            let mut word = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphabetic() || ch.is_digit(10) {
                    word.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            if keywords.contains_key(&word) {
                tokens.push(Token { token_type: TokenType::Keyword, value: word });
            } else {
                tokens.push(Token { token_type: TokenType::Identifier, value: word });
            }
        } else if c.is_digit(10) {  // الأرقام
            let mut num = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_digit(10) {
                    num.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token { token_type: TokenType::Number, value: num });
        } else {
            tokens.push(Token { token_type: TokenType::Symbol, value: c.to_string() });
            chars.next();
        }
    }
    
    tokens.push(Token { token_type: TokenType::EndOfFile, value: "".to_string() });
    tokens
}

// دالة لاختبار الـ Lexer
fn test_lexer(code: &str) {
    let tokens = lexer(code);
    for token in tokens {
        println!("{:?}", token);
    }
}

fn main() {
    use std::io::{self, Write};
    
    print!("أدخل الكود بلغة 'عربي': ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    test_lexer(&input);
}
