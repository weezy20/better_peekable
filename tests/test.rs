#![cfg(test)]
mod tests {
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
    fn iterator_methods_against_peekable() {
        // Peekable
        let mut ap = (0..100).peekable();
        // BetterPeekable
        let mut bp = (0..100).better_peekable();

        assert_eq!(ap.size_hint(), bp.size_hint());
        assert_eq!(ap.next(), bp.next()); // 0
        assert_eq!(ap.size_hint(), bp.size_hint());

        assert_eq!(ap.size_hint(), bp.size_hint());
        assert_eq!(ap.peek(), bp.peek()); // 1
        assert_eq!(ap.size_hint(), bp.size_hint());

        assert_eq!(ap.size_hint(), bp.size_hint());
        let _ = ap.peek();
        let _ = bp.peek_n(10);
        assert_eq!(ap.size_hint(), bp.size_hint());

        assert_eq!(ap.clone().count(), bp.clone().count());
        assert_eq!(ap.nth(5), bp.nth(5)); // 6
        assert_eq!(
            {
                ap.nth(5); // 11
                ap.peek()
            },
            {
                bp.nth(5); // 11
                bp.peek()
            }
        );
        assert_eq!(ap.size_hint(), bp.size_hint());
        assert_eq!(ap.peek(), bp.peek());
        assert_eq!(ap.peek(), bp.peek());
    }

    #[test]
    fn double_ended_basic() {
        let mut bp = (0..100).better_peekable();
        assert_eq!(Some(99), bp.next_back());
        assert_eq!(Some(0), bp.next());
        assert_eq!(Some(98), bp.next_back());
    }

    #[test]
    fn test_peek_n_multiple_positions() {
        let mut iter = (0..20).better_peekable();

        // First, peek at position 2
        assert_eq!(iter.peek_n(2), Some(&2));
        assert_eq!(iter.peek_n(5), Some(&5));

        // Verify we can still peek at intermediate positions
        assert_eq!(iter.peek_n(3), Some(&3));
        assert_eq!(iter.peek_n(4), Some(&4));

        // Now consume one item and try again
        assert_eq!(iter.next(), Some(0));

        // peek_n(7) from current position means we need item at index 8 overall
        // Cache now should have [1, 2, 3, 4, 5]
        // We need to load [6, 7, 8]
        assert_eq!(iter.peek_n(7), Some(&8));

        // Verify all the items in between are still accessible
        assert_eq!(iter.peek_n(0), Some(&1));
        assert_eq!(iter.peek_n(1), Some(&2));
        assert_eq!(iter.peek_n(6), Some(&7));
    }

    #[test]
    fn test_peek_n_after_next() {
        let mut iter = (0..20).better_peekable();

        // Initial peek
        assert_eq!(iter.peek_n(3), Some(&3));

        // Consume items
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));

        assert_eq!(iter.peek_n(5), Some(&7));

        // Verify it actually loaded them
        assert_eq!(iter.peek_n(4), Some(&6));
        assert_eq!(iter.peek_n(3), Some(&5));
    }

    #[test]
    fn test_peek_n_idempotent() {
        let mut iter = (0..20).better_peekable();

        // Multiple calls to same position should return same value
        assert_eq!(iter.peek_n(3), Some(&3));
        assert_eq!(iter.peek_n(3), Some(&3));
        assert_eq!(iter.peek_n(3), Some(&3));

        // And shouldn't prevent peeking further
        assert_eq!(iter.peek_n(7), Some(&7));
        assert_eq!(iter.peek_n(7), Some(&7));

        // Original position should still work
        assert_eq!(iter.peek_n(3), Some(&3));
    }
}
