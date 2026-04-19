use std::{collections::HashSet as StdHashSet};
use crate::trait_structure::StructureDonnee;

#[derive(Clone, Debug)]
pub struct MyHashSet (pub StdHashSet<i32>);


impl StructureDonnee for MyHashSet{
    fn new() -> Self {
        Self(StdHashSet::new())
    }

    fn add(&mut self, value: i32) {
        self.0.insert(value);
    }
    
    fn remove(&mut self, value: i32) {
        self.0.remove(&value);
    }
    
    fn there_is(&self, value: i32) -> bool {
        self.0.contains(&value)
    }
    
    fn map(&self, f: impl Fn(i32) -> i32 + Copy) -> Self {
        MyHashSet(self.0.iter().map(|&v| f(v)).collect())
    }
    
    fn iter(&self) -> Box<dyn Iterator<Item = &i32> + '_> {
        Box::new(self.0.iter())
    }
    
    fn fragmenter(self, taille_max: usize) -> Vec<Self>
    where
        Self: Sized + Clone {
            self.0
            .iter()
            .collect::<Vec<_>>()
            .chunks(taille_max)
            .map(|chunck| Self(chunck.iter().map(|&&v| v).collect()))
            .collect()
    }
    
    fn serialiser(&self) {
        todo!()
    }
    
    fn deserialiser(&self) {
        todo!()
    }

    
}