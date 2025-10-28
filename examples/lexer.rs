use better_peekable::BetterPeekable;

#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Identifier(String),
    Arrow,      // ->
    Minus,      // -
    Greater,    // >
    Equal,      // =
    EqualEqual, // ==
    Plus,       // +
    LeftParen,  // (
    RightParen, // )
    Whitespace,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().better_peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut num = 0;
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        num = num * 10 + (chars.next().unwrap() as i32 - '0' as i32);
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(ident));
            }
            '-' => {
                chars.next(); // consume '-'
                // Look ahead for arrow operator
                if chars.peek() == Some(&'>') {
                    chars.next(); // consume '>'
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '=' => {
                chars.next(); // consume '='
                // Look ahead for equality operator
                if chars.peek() == Some(&'=') {
                    chars.next(); // consume second '='
                    tokens.push(Token::EqualEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '>' => {
                chars.next();
                tokens.push(Token::Greater);
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
                tokens.push(Token::Whitespace);
            }
            _ => {
                chars.next(); // skip unknown chars
            }
        }
    }

    tokens
}

fn main() {
    // Example 1: Basic tokenization
    let input = "x -> 42 == y";
    let tokens = tokenize(input);
    println!("Input: {}", input);
    println!("Tokens: {:?}", tokens);
    println!();

    // Example 2: Demonstrating lookahead with peek_n
    let input2 = "if condition then action else other";
    let mut chars = input2.chars().better_peekable();

    println!("Demonstrating multi-character lookahead:");
    println!("Input: {}", input2);

    // Check if it starts with "if"
    if chars.peek() == Some(&'i') && chars.peek_n(1) == Some(&'f') && chars.peek_n(2) == Some(&' ')
    {
        println!("Found 'if' keyword at start");
        chars.next(); // consume 'i'
        chars.next(); // consume 'f'
        chars.next(); // consume ' '
    }

    // Look for "then" keyword by peeking ahead
    let mut pos = 0;
    while let Some(&ch) = chars.peek_n(pos) {
        if ch == 't'
            && chars.peek_n(pos + 1) == Some(&'h')
            && chars.peek_n(pos + 2) == Some(&'e')
            && chars.peek_n(pos + 3) == Some(&'n')
        {
            println!("Found 'then' keyword at position {}", pos);
            break;
        }
        pos += 1;
    }

    // Example 3: Parser-like usage
    println!("\nParser example:");
    let expr = "func(arg1, arg2)";
    let mut chars = expr.chars().better_peekable();

    // Parse function name
    let mut func_name = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_alphabetic() {
            func_name.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    // Check for function call pattern
    if chars.peek() == Some(&'(') {
        println!("Parsing function call: {}", func_name);
        chars.next(); // consume '('

        let mut arg_count = 0;
        let mut paren_depth = 1;

        while paren_depth > 0 && chars.peek().is_some() {
            match chars.peek() {
                Some(&'(') => {
                    paren_depth += 1;
                    chars.next();
                }
                Some(&')') => {
                    paren_depth -= 1;
                    chars.next();
                }
                Some(&',') if paren_depth == 1 => {
                    arg_count += 1;
                    chars.next();
                }
                _ => {
                    chars.next();
                }
            }
        }

        if arg_count > 0 || func_name.len() > 0 {
            arg_count += 1; // Count the last argument
        }

        println!("Function '{}' has {} arguments", func_name, arg_count);
    }
}
