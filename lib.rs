#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(feature = "alloc", not(test)))]
use alloc::collections::VecDeque;

#[cfg(test)]
use std::collections::VecDeque;

#[cfg(not(feature = "alloc"))]
compile_error!("This crate requires the 'alloc' feature to be enabled for BetterPeekable to work.");

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
