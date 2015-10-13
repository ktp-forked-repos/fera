use std::collections::HashSet;
use std::hash::Hash;

use rand::Rng;

pub struct Map1<'a, I, D: 'a, F> {
    iter: I,
    data: &'a D,
    f: F,
}

impl<'a, A, I, D, F> Iterator for Map1<'a, I, D, F>
    where I:Iterator,
          F: FnMut(&'a D, I::Item) -> A {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|a| (self.f)(self.data, a))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub trait IteratorExt: Iterator + Sized {
    fn map1<D, F>(self, data: &D, f: F) -> Map1<Self, D, F> {
        Map1 { iter: self, data: data, f: f }
    }

    fn into_vec(self) -> Vec<Self::Item> {
        self.collect()
    }

    fn into_set(self) -> HashSet<Self::Item>
        where Self::Item: Hash + Eq
    {
        self.collect()
    }

    fn into_shuffled_vec<R: Rng>(self, rng: &mut R) -> Vec<Self::Item> {
        let mut v = self.into_vec();
        rng.shuffle(&mut v[..]);
        v
    }
}

impl<I: Iterator> IteratorExt for I {}
