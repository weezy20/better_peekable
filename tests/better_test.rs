#![cfg(test)]
use better_peekable::BetterPeekable;

#[test]
fn better_peekable_api() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // let len = vec.len();
    // dbg!(len);
    let iter = vec.into_iter();
    let mut better_peeker = iter.better_peekable();
    // peek_n(8) when vector length == 8 should be out of bounds
    assert_eq!(better_peeker.peek_n(8), None);
    assert_eq!(better_peeker.peek_n(7), Some(&8));
    assert_eq!(better_peeker.peek(), Some(&1));
    assert_eq!(better_peeker.peek_n(1), Some(&2));
    assert_eq!(better_peeker.peek_n(2), Some(&3));
    assert_eq!(better_peeker.peek(), Some(&1));
    assert_eq!(better_peeker.peek_n(2), Some(&3));
    assert_eq!(better_peeker.next(), Some(1));
    assert_eq!(better_peeker.peek(), Some(&2));
    assert_eq!(better_peeker.next(), Some(2));
    assert_eq!(better_peeker.peek(), Some(&3));
}

#[test]
fn better_peekable_heap_data() {
    let vec = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("It's a nice day to make"),
        String::from("A better peekable iterator adaptor"),
        String::from("Peek_N and Peek are supposed to be"),
        String::from("Idempotent Methods"),
    ];
    let len = vec.len();
    dbg!(len);
    let iter = vec.into_iter();
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
}

#[test]
fn iterations() {
    let mut bp = (0..100).better_peekable();
    let mut ap = (0..100).peekable();
    assert_eq!(ap.next(), bp.next());
    assert_eq!(ap.next(), bp.next());
    assert_eq!(ap.next(), bp.next());
    assert_eq!(ap.next(), bp.next());
    assert_eq!(ap.next(), bp.next());
    assert_eq!(ap.next(), bp.next());
    assert_eq!(ap.next(), bp.next());
}

#[test]
fn iterator_methods() {
    let mut bp = (0..100).better_peekable();
    let mut ap = (0..100).peekable();

    assert_eq!(ap.clone().count(), bp.clone().count());
    assert_eq!(ap.nth(5), bp.nth(5));
    assert_eq!(
        {
            ap.nth(5);
            ap.peek()
        },
        {
            bp.nth(5);
            bp.peek()
        }
    );
    assert_eq!(ap.rposition(|x| x == 50), bp.rposition(|x| x == 50));
}
