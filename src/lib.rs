use std::collections::VecDeque;
use std::marker::PhantomData;

/// A better peekable struct unlike the std version of Peekable, where we hold more than 1 peeked char
pub struct BPeekable<I: Iterator> {
    iter: PhantomData<I>,
    /// A cache holding a small list of peekables into the future
    /// Option<I::Item> is yielded by inner iterator
    cache: VecDeque<I::Item>,
}
impl<I: Iterator> BPeekable<I> {
    pub fn new(i: I) -> Self {
        let cache: VecDeque<_> = i.collect();
        Self {
            iter: PhantomData,
            cache,
        }
    }
}

pub trait BetterPeekable<I: Iterator> {
    /// Peek once, just like Peekable
    fn peek(&mut self) -> Option<&I::Item>;
    /// Peek `n` items into the Iterator
    /// peek() and peek_n(0) are equivalent
    fn peek_n(&mut self, n: usize) -> Option<&I::Item>;
}

impl<I> BetterPeekable<I> for BPeekable<I>
where
    I: Iterator,
{
    /// Peek once
    fn peek(&mut self) -> Option<&I::Item> {
        self.cache.get(0)
    }

    fn peek_n(&mut self, n: usize) -> Option<&I::Item> {
        self.cache.get(n)
    }
}

impl<I> Iterator for BPeekable<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.cache.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn better_peekable_api() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let len = vec.len();
        dbg!(len);
        let mut iter = vec.into_iter();
        let mut better_peeker = BPeekable::new(iter);
        // peek_n(8) when vector length == 8 should be out of bounds
        assert_eq!(better_peeker.peek_n(8), None);
        assert_eq!(better_peeker.peek_n(7), Some(&8));
        assert_eq!(better_peeker.peek_n(2), Some(&3));
        assert_eq!(better_peeker.peek(), Some(&1));
        assert_eq!(better_peeker.peek_n(2), Some(&3));
        assert_eq!(better_peeker.next(), Some(1));
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
}
