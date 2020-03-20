#![crate_name="zipwith"]

use std::cmp::min;
use std::iter::{Iterator,IntoIterator};

#[cfg(test)]
mod tests 
{
    use super::*;
    use std::cmp::max;

    #[test]
    fn zip_with_maxes_func() {
        let left = vec![0, 44, -12];
        let right = vec![4, 5, -8];
        let result: Vec<i8> = zip_with(left.into_iter(), right.into_iter(), max).collect();
        
        assert_eq!(result, vec![4, 44, -8]);
    }

    #[test]
    fn zip_with_maxes_meth() {
        let left = vec![1, 2, 9];
        let right = vec![4, 5, 6];
        let result: Vec<&u8> = left.iter().zip_with(right.iter(), max).collect();
        
        assert_eq!(result, vec![&4, &5, &9]);
    }
}

/// ZipWith struct
#[derive(Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct ZipWith<T, U, F> {
    left: T,
    right: U,
    zipper: F
}

/// Create a new zip_with iterator.
/// 
/// # Params
/// * left The left-side iterator to zip.
/// * right The right-side iterator to zip.
/// * zipper The zipper function.
/// 
/// # Return
/// A ZipWith iterator containing the result of applying the zipper function per-element.
/// The resulting iterator will be as large as the smallest of the two iterators given.
pub fn zip_with<T, U, R, F>(left: T, right: U, zipper: F) -> ZipWith<T, U, F> where
    T: Iterator,
    U: Iterator, 
    F: Fn(T::Item, U::Item) -> R
{
    ZipWith { left: left.into_iter(), right: right.into_iter(), zipper: zipper }
}

// The IntoZipWith trait
pub trait IntoZipWith: IntoIterator + Sized 
{
    fn zip_with<R, F, S>(self, other: R, zipper: F) -> ZipWith<Self::IntoIter, R::IntoIter, F> where 
        R: Sized + IntoIterator,
        F: Fn(Self::Item, R::Item) -> S
    {
        zip_with(self.into_iter(), other.into_iter(), zipper)
    }
}

/// Iterator implmentation
impl<T, U, R, F> Iterator for ZipWith<T, U, F> where
    T: Iterator,
    U: Iterator,
    F: Fn(T::Item, U::Item) -> R
{
    type Item = R;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> 
    {
        match self.left.next()
        {
            Some(l) => match self.right.next()
            {
                Some(r) => Some((self.zipper) (l, r)),
                None => None
            },
            None => None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) 
    {
        let left = self.left.size_hint();
        let right = self.right.size_hint();
        (min(left.0,right.0), match left.1
        {
            Some(l) => match right.1 
            {
                Some(r) => Some(min(l, r)),
                None => Some(l)
            },
            None => match right.1 
            {
                Some(r) => Some(r),
                None => None
            }
        })
    }
}

/// Iterators implement IntoZipWith
impl<T: Iterator> IntoZipWith for T {}
