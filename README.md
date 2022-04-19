# A Better Peekable

This crate provides a trait `BetterPeekable` and a type `BPeekable<I: Iterator>` which is a wrapper over an iterator and returned by calling `better_peekable()` on any `Iterator`. Which implies that you can call the usual iterator methods on `BPeekable<I>` like `next()` and others (ExactSizedIterator and DoubleEndedIterator are unimplemented at the time of version 0.2.0, it will be implemented in v0.2.1), in addition to `peek` and `peek_n` which allow you to peek the inner iterator without consuming it. `peek` gives you a reference to the immediately available item to be consumed by a `next()` call, whereas `peek_n` allows you to peek `n` times ahead. Calling `peek_n(0)` is the same as calling `peek`.

`peek` and `peek_n` are idempotent which means calling them repeatedly on `BPeekable` should have no effects on the underlying iterator or any state of `BPeekable`.

## Usage

Add to `better_peekable` to your `Cargo.toml`.
I recommend installing `cargo-edit` as it makes adding crates easier.

```shell
cargo add better_peekable
```

We are going to test the idempotence of `BPeekable` using the following sequence of `peek` and `peek_n` calls. 
Feel free to file an issue if you test and find out if something doesn't match up.

```rust
// Required for peek and peek_n 
use better_peekable::BetterPeekable;

fn main() {
    let vec = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("It's a nice day to make"),
            String::from("A better peekable iterator adaptor"),
            String::from("Peek_N and Peek are supposed to be"),
            String::from("Idempotent Methods"),
        ];

        let mut iter = vec.into_iter();
        let mut better_peeker = iter.better_peekable();
       
        assert_eq!(better_peeker.peek(), Some(&"Hello".to_string()));
        assert_eq!(better_peeker.peek(), Some(&"Hello".to_string()));
        assert_eq!(better_peeker.peek(), Some(&"Hello".to_string()));
        assert_eq!(better_peeker.peek_n(1), Some(&"World".to_string()));
        assert_eq!(better_peeker.next(), Some("Hello".to_string()));
        assert_eq!(better_peeker.next(), Some("World".to_string()));
        assert_eq!(
            better_peeker.peek(),
            Some(&"It's a nice day to make".to_string())
        );
        assert_eq!(
            better_peeker.peek_n(0),
            Some(&"It's a nice day to make".to_string())
        );
        assert_eq!(
            better_peeker.peek_n(1),
            Some(&"A better peekable iterator adaptor".to_string())
        );
        assert_eq!(
            better_peeker.nth(1),
            Some("A better peekable iterator adaptor".to_string())
        );
        assert_eq!(
            better_peeker.peek_n(1),
            Some(&"Idempotent Methods".to_string())
        );
```

