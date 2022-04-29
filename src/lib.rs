use std::collections::VecDeque;

/// An extension trait for `Iterator`s allowing better peeking into the iterator
pub trait BetterPeekable: Iterator
where
    Self: Sized,
{
    fn better_peekable(self) -> BPeekable<Self> {
        init::<Self>(self)
    }
}

/// A better peekable struct unlike the std version of Peekable, where we hold more than 1 peeked item.
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
{
    pub fn new(i: I) -> Self {
        let cache: VecDeque<Option<I::Item>> = VecDeque::new();
        let iter: I = i;

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
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.is_empty() {
            self.peek();
        }
        // reset peek
        self.peeked = false;
        self.cache.pop_front()?
    }

    #[inline]
    /// Returns the bounds on the remaining length of the iterator.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let peek_len = self.cache.len();
        let (lo, hi) = self.iter.size_hint();
        let lo = lo.saturating_add(peek_len);
        let hi = match hi {
            Some(x) => x.checked_add(peek_len),
            None => None,
        };
        (lo, hi)
    }
}

impl<I> BetterPeekable for I where I: Iterator {}

impl<I: Iterator> ExactSizeIterator for BPeekable<I> {}

impl<I> DoubleEndedIterator for BPeekable<I>
where
    I: DoubleEndedIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item>
    {
        match self.cache.back_mut() {
            // Cache has a Some(_) at its back, call interior next_back or else return this Some(_)
            Some(item @ Some(_)) => self.iter.next_back().or_else(|| item.take()),
            // Cache has None in it, therefore the interior iterator has yielded
            Some(None) => None,
            // Cache is empty, check inner iterator
            None => self.iter.next_back(),
        }
    }
}
pub fn init<I: Iterator>(i: I) -> BPeekable<I> {
    BPeekable::new(i)
}
