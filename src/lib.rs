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


