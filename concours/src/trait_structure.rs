use std::clone;

pub trait structure_donnee{
    fn new() -> Self;

    fn add(x : i32) -> Self;

    fn remove(x : i32) -> Self;

    fn delete();

    fn fragmenter(&self, taille_max : usize) -> Vec<Self> where Self: Sized + Clone;

    fn serialiser();

    fn deserialiser();

}