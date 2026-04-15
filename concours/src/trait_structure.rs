use std::clone;

pub trait structure_donnee {
    fn new() -> Self;

    fn add(&mut self, value: i32);

    fn remove(&mut self, value: i32);

    fn there_is(&self, value: i32) -> bool;

    fn map(&self, f: impl Fn(i32) -> i32 + Copy) -> Self;

    fn iter(&self) -> Box<dyn Iterator<Item = &i32> + '_>;

    // separer l'ensemble en liste de plus petit ensemble
    fn fragmenter(self, taille_max: usize) -> Vec<Self>
    where
        Self: Sized + Clone;

    fn intersection(self, other: Self) -> Self
    where
        Self: Sized,
    {
        let mut iter = other.iter();
        let mut new = Self::new();
        while let Some(value) = iter.next() {
            if self.there_is(*value) {
                new.add(*value);
            }
        }
        new
    }

    fn union(&mut self, other: &Self)
    where
        Self: Sized,
    {
        let mut iter = other.iter();
        while let Some(value) = iter.next() {
            self.add(*value);
        }
    }

    fn difference(&mut self, other: &mut Self)
    where
        Self: Sized,
    {
        let mut iter = other.iter();
        while let Some(value) = iter.next() {
            if self.there_is(*value) {
                self.remove(*value);
            }
        }
    }

    fn difference_symetrique(&mut self, other: &mut Self)
    where
        Self: Sized,
    {
        let mut iter = other.iter();
        while let Some(value) = iter.next() {
            if self.there_is(*value) {
                self.remove(*value);
                // other.remove(*value);
            }
        }
    }

    fn serialiser(&self);

    fn deserialiser(&self);
}
