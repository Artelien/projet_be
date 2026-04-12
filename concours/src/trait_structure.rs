use std::clone;

pub trait structure_donnee{
    fn new(x : i32) -> Self;

    fn add(&mut self, x : i32);

    fn remove(&mut self, x : i32);

    fn map(&self,  f: impl Fn(i32) -> i32 + Copy) -> Self;

    fn fragmenter(&self, taille_max : usize) -> Vec<Self> where Self: Sized + Clone;

    fn serialiser(&self);

    fn deserialiser(&self);

}