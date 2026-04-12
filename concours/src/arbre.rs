use crate::trait_structure::{self, structure_donnee};


#[derive (Clone, Debug)]
pub struct arbre{
    pub value: i32,
    pub left : Option<Box<arbre>>,
    pub right :  Option<Box<arbre>>
}


impl structure_donnee for arbre {
    fn add(x : i32) {
        todo!()
    }

    fn remove(x : i32) {
        todo!()
    }

    fn delete() {
        todo!()
    }

    fn fragmenter() {
        todo!()
    }

    fn serialiser() {
        todo!()
    }

    fn deserialiser() {
        todo!()
    }
}