use std::collections::BTreeSet;

use crate::StructureDonnee;

#[derive(Clone, Debug)]
pub struct MyBTree(pub BTreeSet<i32>);

impl StructureDonnee for MyBTree{
    fn new() -> Self {
        Self(BTreeSet::new())
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
        MyBTree(self.0.iter().map(|&v| v).collect())
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