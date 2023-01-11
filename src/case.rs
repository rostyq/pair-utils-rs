use crate::side::Side;
use crate::pair::Pair;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Case<T> {
    Both(Pair<T>),
    One(Side, T),
    None,
}

impl<T> From<[Option<T>; 2]> for Case<T> {
    fn from(arr: [Option<T>; 2]) -> Self {
        match arr {
            [Some(left), Some(right)] => Self::Both([left, right].into()),
            [Some(left), None] => Self::One(Side::L, left),
            [None, Some(right)] => Self::One(Side::R, right),
            [None, None] => Self::None,
        }
    }
}

impl<T> From<Case<T>> for [Option<T>; 2] { 
    fn from(state: Case<T>) -> Self {
        match state {
            Case::Both(pair) => {
                let [left, right]: [T; 2] = pair.into();
                [Some(left), Some(right)]
            },
            Case::One(pair, value) => {
                match pair {
                    Side::L => [Some(value), None],
                    Side::R => [None, Some(value)],
                }
            },
            Case::None => [None, None],
        }
        
    }
}

impl<T> From<Pair<Option<T>>> for Case<T> {
    fn from(pair: Pair<Option<T>>) -> Self {
        Into::<[Option<T>; 2]>::into(pair).into()
    }
}

impl<T> From<Case<T>> for Pair<Option<T>> { 
    fn from(state: Case<T>) -> Self {
        Into::<[Option<T>; 2]>::into(state).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_none() {
        let avail: Case<()> = [None, None].into();
        assert_eq!(avail, Case::None);
    }
}