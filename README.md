# A Better Peekable

```rust
use better_peekable::{BPeekable, BetterPeekable};

fn main() {
     let vec = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("It's a nice day to make"),
            String::from("A better peekable iterator adaptor"),
        ];
        let len = vec.len();
        dbg!(len);
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