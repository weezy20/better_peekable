# Better Peekable

A `no_std` compatible iterator adapter that lets you peek multiple items ahead - perfect for building lexers, parsers, and other lookahead-heavy code.

Unlike `std::iter::Peekable` which only lets you peek one item ahead, `BetterPeekable` provides `peek_n(n)` to look arbitrarily far into your iterator without consuming items. Of course it also provides `peek()` which is the same as `peek_n(0)`.

## Installation

```sh
cargo add better_peekable
```

For `no_std` environments:

```sh
cargo add better_peekable --no-default-features --features alloc
```

## Quick Example

```rust
use better_peekable::BetterPeekable;

let mut tokens = "if x == 42".chars().better_peekable();

// Look ahead to parse keywords
if tokens.peek() == Some(&'i') && tokens.peek_n(1) == Some(&'f') {
    tokens.next(); // consume 'i'
    tokens.next(); // consume 'f'
    println!("Found 'if' keyword");
}
```

## Lexer Example

Here's how you might use it to build a simple lexer that needs lookahead:

```rust
use better_peekable::BetterPeekable;

#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Arrow,      // ->
    Minus,      // -
    Greater,    // >
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
            '>' => {
                chars.next();
                tokens.push(Token::Greater);
            }
            ' ' => { chars.next(); } // skip whitespace
            _ => { chars.next(); }   // skip unknown chars
        }
    }

    tokens
}

fn main() {
    let tokens = tokenize("42 -> 7 - 3");
    println!("{:?}", tokens);
    // Output: [Number(42), Arrow, Number(7), Minus, Number(3)]
}
```

## Key Features

- **Multi-item lookahead**: `peek_n(n)` lets you look `n` items ahead
- **Idempotent**: Multiple `peek` calls don't affect the iterator state
- **Full iterator support**: Works with `DoubleEndedIterator`, `ExactSizeIterator`, etc.
- **`no_std` compatible**: Works in embedded and WASM environments with `alloc`
