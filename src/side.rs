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

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn swap_left_to_right() {
        assert_eq!(Side::L.swap(), Side::R);
    }

    #[test]
    fn swap_right_to_left() {
        assert_eq!(Side::R.swap(), Side::L);
    }

    #[test]
    fn left_as_number_0() {
        assert_eq!(Side::L as usize, 0);
    }

    #[test]
    fn right_as_number_1() {
        assert_eq!(Side::R as usize, 1);
    }

    #[test]
    fn left_from_number_0() {
        assert_eq!(Side::from(0), Side::L);
    }

    #[test]
    fn right_from_number_1() {
        assert_eq!(Side::from(1), Side::R);
    }

    #[test]
    fn panic_if_created_from_number_larger_than_1() {
        let hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        assert!(panic::catch_unwind(|| Side::from(2)).is_err());
        panic::set_hook(hook);
    }
}