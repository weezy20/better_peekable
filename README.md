# A Better Peekable

This crate provides `BPeekable<I: Iterator>` which is a wrapper over an iterator. You can call the usual iterator methods on `BPeekable<I>` like `next()` and others (untested at the moment), in addition to `peek()` and `peek_n(n: usize)` which allow you to peek the inner iterator without consuming it. `peek()` gives you a reference to the immediately available item to be consumed by a `next()` call, whereas `peek_n(n: usize)` allows you to peek `n` times ahead. Calling `peek_n(0)` is the same as calling `peek`.

## Usage

```rust
use better_peekable::{BPeekable, BetterPeekable};

fn main() {
     let vec = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("It's a nice day to make"),
            String::from("A better peekable iterator adaptor"),
        ];

        let mut iter = vec.into_iter();
        let mut better_peeker = BPeekable::new(iter);

        assert_eq!(better_peeker.peek(), Some(&"Hello".to_string()));
        assert_eq!(better_peeker.peek(), Some(&"Hello".to_string()));
        assert_eq!(better_peeker.peek(), Some(&"Hello".to_string()));
        assert_eq!(better_peeker.peek_n(1), Some(&"World".to_string()));
        assert_eq!(better_peeker.next(), Some("Hello".to_string()));
        assert_eq!(better_peeker.next(), Some("World".to_string()));
        assert_eq!(better_peeker.peek_n(1), Some(&"A better peekable iterator adaptor".to_string()));
}
