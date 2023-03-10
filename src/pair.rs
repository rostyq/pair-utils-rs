use crate::paired::Paired;
use crate::side::Side;
use std::{
    convert::From,
    ops::{Index, IndexMut},
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Pair<T>([T; 2]);

impl<T> Paired for Pair<T> {
    type Item = T;
    fn swap(&mut self) {
        let data_ptr = self.0.as_mut_ptr();
        unsafe {
            std::ptr::swap(data_ptr.add(0), data_ptr.add(1));
        }
    }

    fn get_left(&self) -> &Self::Item {
        unsafe { self.0.get_unchecked(0) }
    }

    fn get_right(&self) -> &Self::Item {
        unsafe { self.0.get_unchecked(1) }
    }

    fn get_left_mut(&mut self) -> &mut Self::Item {
        unsafe { self.0.get_unchecked_mut(0) }
    }

    fn get_right_mut(&mut self) -> &mut Self::Item {
        unsafe { self.0.get_unchecked_mut(1) }
    }
}

impl<T> Pair<T> {
    pub fn new(left: T, right: T) -> Self {
        Self([left, right])
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.0.iter_mut()
    }

    pub fn map<U, F>(self, f: F) -> Pair<U>
    where
        F: Fn(T) -> U,
    {
        self.0.map(f).into()
    }

    pub fn zip<U>(self, other: Pair<U>) -> Pair<(T, U)> {
        let [l1, r1] = self.0;
        let [l2, r2]: [U; 2] = other.into();
        [(l1, l2), (r1, r2)].into()
    }

    pub fn compare<'a, U>(self, other: &'a Pair<U>) -> Pair<(T, &'a U)> {
        let [l1, r1] = self.0;
        [(l1, other.get_left()), (r1, other.get_right())].into()
    }

    pub fn merge<U, V, F>(self, other: Pair<U>, f: F) -> Pair<V>
    where
        F: Fn(T, U) -> V,
    {
        self.zip(other).map(|(t, u)| f(t, u))
    }

    pub fn apply<U, V, F>(self, other: &Pair<U>, f: F) -> Pair<V>
    where
        F: Fn(T, &U) -> V,
    {
        self.compare(other).map(|(t, u)| f(t, u))
    }
}

impl<T> Index<Side> for Pair<T> {
    type Output = T;

    fn index<'a>(&'a self, index: Side) -> &'a Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<Side> for Pair<T> {
    fn index_mut<'a>(&'a mut self, index: Side) -> &'a mut T {
        self.get_mut(index)
    }
}

impl<T> IntoIterator for Pair<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> AsRef<[T; 2]> for Pair<T> {
    fn as_ref(&self) -> &[T; 2] {
        &self.0
    }
}

impl<T> AsMut<[T; 2]> for Pair<T> {
    fn as_mut(&mut self) -> &mut [T; 2] {
        &mut self.0
    }
}

impl<T> From<[T; 2]> for Pair<T> {
    fn from(arr: [T; 2]) -> Self {
        Pair(arr)
    }
}

impl<T> From<Pair<T>> for [T; 2] {
    fn from(pair: Pair<T>) -> Self {
        pair.0
    }
}

impl<T> From<Pair<T>> for Pair<Option<T>> {
    fn from(pair: Pair<T>) -> Self {
        let [left, right]: [T; 2] = pair.into();
        Self::from([Some(left), Some(right)])
    }
}

impl<T> Pair<Option<T>> {
    pub fn both(&self) -> bool {
        self.get_left().is_some() && self.get_right().is_some()
    }

    pub fn none(&self) -> bool {
        self.get_left().is_none() && self.get_right().is_none()
    }

    pub fn any(&self) -> bool {
        self.get_left().is_some() || self.get_right().is_some()
    }

    pub fn one(&self) -> bool {
        if self.get_left().is_some() {
            self.get_right().is_none()
        } else {
            self.get_right().is_some()
        }
    }

    pub fn map_some<U, F>(self, f: F) -> Pair<Option<U>>
    where
        F: Fn(T) -> U,
    {
        self.map(|i| i.map(|v| f(v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_into_array() {
        let (left, right) = (0, 1);
        let pair = Pair::from([left, right]);
        assert_eq!(pair[Side::L], left);
        assert_eq!(pair[Side::R], right);

        let arr: [i32; 2] = pair.into();
        assert_eq!(arr, [left, right]);
    }

    #[test]
    fn index_by_side() {
        let (left, right) = (0, 1);
        let mut pair = Pair::new(left, right);

        assert_eq!(pair[Side::L], left);
        assert_eq!(pair[Side::R], right);

        let (new_left, new_right) = (0, 1);
        pair[Side::L] = new_left;
        pair[Side::R] = new_right;

        assert_eq!(pair[Side::L], new_left);
        assert_eq!(pair[Side::R], new_right);
    }

    #[test]
    fn swap() {
        let (left, right) = (0, 1);
        let mut pair = Pair::new(left, right);
        pair.swap();

        assert_eq!(pair, Pair::from([right, left]));
    }

    #[test]
    fn into_iter() {
        let (left, right) = (0, 1);
        let pair = Pair::new(left, right);

        let mut it = pair.into_iter();

        assert_eq!(it.next().unwrap(), left);
        assert_eq!(it.next().unwrap(), right);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn option() {
        let (left, right) = (Some(0), Some(1));
        let both_pair = Pair::new(left, right);
        let none_pair: Pair<Option<()>> = Pair::new(None, None);
        let left_pair = Pair::new(left, None);
        let right_pair = Pair::new(None, right);

        assert!(both_pair.both());
        assert!(both_pair.any());
        assert!(!both_pair.none());
        assert!(!both_pair.one());

        assert!(!none_pair.both());
        assert!(!none_pair.any());
        assert!(none_pair.none());
        assert!(!none_pair.one());

        assert!(!left_pair.both());
        assert!(left_pair.any());
        assert!(!left_pair.none());
        assert!(left_pair.one());

        assert!(!right_pair.both());
        assert!(right_pair.any());
        assert!(!right_pair.none());
        assert!(right_pair.one());
    }
}
