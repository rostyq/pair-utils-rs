use crate::Side;

pub trait Paired {
    type Item;

    fn get_left(&self) -> &Self::Item;

    fn get_right(&self) -> &Self::Item;

    fn get_left_mut(&mut self) -> &mut Self::Item;

    fn get_right_mut(&mut self) -> &mut Self::Item;

    fn get(&self, s: Side) -> &Self::Item {
        match s {
            Side::L => self.get_left(),
            Side::R => self.get_right(),
        }
    }

    fn get_mut(&mut self, s: Side) -> &mut Self::Item {
        match s {
            Side::L => self.get_left_mut(),
            Side::R => self.get_right_mut(),
        }
    }

    fn swap(&mut self);
}