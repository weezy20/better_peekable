#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(feature = "alloc", not(test)))]
use alloc::collections::VecDeque;

#[cfg(test)]
use std::collections::VecDeque;

#[cfg(not(feature = "alloc"))]
compile_error!("This crate requires the 'alloc' or 'std' to be enabled for BetterPeekable to work.");

/// An extension trait for `Iterator`s allowing better peeking into the iterator
pub trait BetterPeekable: Iterator
where
    Self: Sized,
{
    fn better_peekable(self) -> BPeekable<Self> {
        BPeekable::new(self)
    }
}

/// A better peekable struct unlike the std version of Peekable, where we hold more than 1 peeked item.
#[derive(Clone, Debug)]
pub struct BPeekable<I: Iterator> {
    iter: I,
    /// A cache holding items peeked from the iterator
    cache: VecDeque<I::Item>,
}

impl<I: Iterator> BPeekable<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            cache: VecDeque::new(),
        }
    }

    /// Peek at the next item (equivalent to peek_n(0))
    #[inline]
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_n(0)
    }

    /// Peek at the item n positions ahead (0-indexed)
    /// peek_n(0) is the next item, peek_n(1) is the item after that, etc.
    #[inline]
    pub fn peek_n(&mut self, n: usize) -> Option<&I::Item> {
        // Fill cache up to position n if needed
        while self.cache.len() <= n {
            match self.iter.next() {
                Some(item) => self.cache.push_back(item),
                None => return None,
            }
        }
        self.cache.get(n)
    }
}

impl<I: Iterator> Iterator for BPeekable<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.cache.pop_front().or_else(|| self.iter.next())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let cache_len = self.cache.len();
        let (lo, hi) = self.iter.size_hint();
        let lo = lo.saturating_add(cache_len);
        let hi = hi.and_then(|h| h.checked_add(cache_len));
        (lo, hi)
    }
}

impl<I: Iterator> BetterPeekable for I {}

impl<I: ExactSizeIterator> ExactSizeIterator for BPeekable<I> {
    #[inline]
    fn len(&self) -> usize {
        self.cache.len() + self.iter.len()
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for BPeekable<I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.cache.pop_back().or_else(|| self.iter.next_back())
    }
}

pub fn init<I: Iterator>(i: I) -> BPeekable<I> {
    BPeekable::new(i)
}

#[cfg(test)]
mod tests {
    use crate::BetterPeekable;
    use rand::prelude::*;

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
    fn double_ended_iterator_methods_against_standard_peekable() {
        // Normal iterator
        let mut ap = (0..100).peekable();
        let mut bp = (0..100).better_peekable();
        assert_eq!(ap.next_back(), bp.next_back());
        assert_eq!(ap.nth_back(10), bp.nth_back(10));
        assert_eq!(ap.rposition(|x| x == 50), bp.rposition(|x| x == 50));
    }

    #[test]
    fn double_ended_iterator_methods_against_standard_iterator() {
        let mut rng = StdRng::seed_from_u64(123);
        // Normal iterator
        let mut ni = 0..100;
        let mut bp = (0..100).better_peekable();
        let (ni_back, bp_back) = (ni.next_back(), bp.next_back());
        assert_eq!(ni_back, bp_back);
        assert_eq!(ni.next(), bp.next());
        assert_eq!(ni.next(), bp.next());
        assert_eq!(ni.size_hint(), bp.size_hint());
        assert_eq!(ni.position(|x| x == 50), bp.position(|x| x == 50));
        assert_eq!(ni.rposition(|x| x == 26), bp.rposition(|x| x == 26));
        assert_eq!(ni.next_back(), bp.next_back());
        let x = rng.random_range(0..100_usize);
        assert_eq!(ni.nth_back(x), bp.nth_back(x));
        let x = rng.random_range(0..100_usize);
        assert_eq!(ni.nth_back(x), bp.nth_back(x));
        assert_eq!(ni.rposition(|x| x == 50), bp.rposition(|x| x == 50));

        assert_eq!(ni.size_hint(), bp.size_hint());
    }

    #[test]
    fn double_ended_sanity_check() {
        let mut ni = 0..100;
        assert_eq!(Some(99), ni.next_back());

        let mut ap = (0..100).peekable();
        assert_eq!(Some(99), ap.next_back());

        let mut bp = (0..100).better_peekable();
        assert_eq!(Some(99), bp.next_back());

        let vec = vec![format!("1"), format!("2"), format!("3"), format!("4")];
        let location_1 = vec[0].as_ptr();
        let mut bap = vec.into_iter().better_peekable();
        // let mut nap = vec.clone().into_iter().peekable();

        assert_eq!(location_1, bap.peek().unwrap().as_ptr());
    }

    // NEW TESTS FOR peek_n BUG

    #[test]
    fn test_peek_n_multiple_positions_bug() {
        // This test demonstrates the bug with the peeked flag
        let mut iter = (0..20).better_peekable();

        // First, peek at position 2
        assert_eq!(iter.peek_n(2), Some(&2));
        println!("After peek_n(2), cache should have [0, 1, 2]");

        // Now try to peek at position 5 - this SHOULD load more items
        // Bug: If peeked flag stays true, it won't load positions 3, 4, 5
        assert_eq!(iter.peek_n(5), Some(&5));
        println!("After peek_n(5), cache should have [0, 1, 2, 3, 4, 5]");

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
    fn test_peek_n_increasing_sequence() {
        // Test that shows the flag isn't reset properly
        let mut iter = (0..100).better_peekable();

        // Peek at increasing positions
        assert_eq!(iter.peek_n(1), Some(&1));
        // peeked is now true, cache has [0, 1]

        assert_eq!(iter.peek_n(5), Some(&5));
        // Bug: peeked is already true, so the for loop doesn't run
        // Cache still only has [0, 1], so peek_n(5) returns None or fails

        assert_eq!(iter.peek_n(10), Some(&10));
        // Same issue - won't load more items
    }

    #[test]
    fn test_peek_n_after_next() {
        let mut iter = (0..20).better_peekable();

        // Initial peek
        assert_eq!(iter.peek_n(3), Some(&3));
        // peeked = true, cache = [0, 1, 2, 3]

        // Consume items
        assert_eq!(iter.next(), Some(0)); // peeked reset to false
        assert_eq!(iter.next(), Some(1)); // peeked reset to false
                                          // cache now = [2, 3]

        // Now peek further ahead - needs to load more items
        assert_eq!(iter.peek_n(5), Some(&7));
        // We need item at absolute position 7 (2 + 5)
        // Cache should become [2, 3, 4, 5, 6, 7]

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
