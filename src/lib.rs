use std::collections::VecDeque;

/// A better peekable struct unlike the std version of Peekable, where we hold more than 1 peeked char.
#[derive(Clone, Debug)]
pub struct BPeekable<I: Iterator> {
    peeked: bool,
    iter: I,
    /// A cache holding a small list of peekables into the future
    /// Option<I::Item> is yielded by inner iterator
    cache: VecDeque<Option<I::Item>>,
}
impl<I: Iterator> BPeekable<I>
where
    I: Iterator,
    //I::Item: std::fmt::Debug,
{
    pub fn new(i: I) -> Self {
        let mut cache: VecDeque<Option<I::Item>> = VecDeque::new();
        let mut iter: I = i;
        // Initialize the cache with one element to allow peeking
        cache.push_back(iter.next());
        Self {
            iter,
            cache,
            peeked: false,
        }
    }
    /// Peek once
    #[inline(always)]
    pub fn peek(&mut self) -> Option<&I::Item> {
        match self.cache.get(0) {
            Some(_) => self.cache.get(0).unwrap().as_ref(),
            None => {
                if let Some(inner_item) = self.iter.next() {
                    self.cache.push_back(Some(inner_item));
                }
                self.cache.get(0)?.as_ref()
            }
        }
    }
    #[inline(always)]
    pub fn peek_n(&mut self, n: usize) -> Option<&I::Item> {
        // Load cache only if we haven't peeked_n into it.
        if !self.peeked {
            for _ in 0..=n {
                if let Some(inner_item) = self.iter.next() {
                    self.cache.push_back(Some(inner_item));
                }
            }
            self.peeked = true;
        }
        self.cache.get(n)?.as_ref()
    }
}


impl<I> Iterator for BPeekable<I>
where
    I: Iterator,
    I::Item: std::fmt::Debug,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.is_empty() {
            self.peek();
        }
        // reset peek
        self.peeked = false;
        self.cache.pop_front()?
    }
}

/// To enable all types implementing iterator to generate BPeekable
pub trait BetterPeekable: Iterator
where
    Self: Sized,
{
    fn better_peekable(self) -> BPeekable<Self> {
        init::<Self>(self)
    }
}

impl<I> BetterPeekable for I where I: Iterator {}

pub fn init<I: Iterator>(i: I) -> BPeekable<I> {
    BPeekable::new(i)
}


#[cfg(test)]
mod tests {
    use super::BetterPeekable;
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
}
