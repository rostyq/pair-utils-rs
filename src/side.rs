#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Side {
    L = 0,
    R = 1,
}

impl Side {
    pub fn swap(self) -> Self {
        match self {
            Self::L => Self::R,
            Self::R => Self::L,
        }
    }
}

impl From<usize> for Side {
    fn from(i: usize) -> Self {
        match i {
            0 => Self::L,
            1 => Self::R,
            _ => panic!("Cannot convert `{}` into `Side`. Should be `0` or `1`", i),
        }
    }
}

impl From<Side> for usize {
    fn from(value: Side) -> Self {
        match value {
            Side::L => 0,
            Side::R => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn swap() {
        assert_eq!(Side::L.swap(), Side::R);
        assert_eq!(Side::R.swap(), Side::L);
    }

    #[test]
    fn as_usize() {
        assert_eq!(Side::L as usize, 0usize);
        assert_eq!(Side::R as usize, 1usize);
    }

    #[test]
    fn from_into_usize() {
        assert_eq!(Side::from(0), Side::L);
        assert_eq!(Side::from(1), Side::R);

        let hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        assert!(panic::catch_unwind(|| Side::from(2)).is_err());
        panic::set_hook(hook);

        let left: usize = Side::L.into();
        assert_eq!(left, 0);

        let right: usize = Side::R.into();
        assert_eq!(right, 1);
    }
}