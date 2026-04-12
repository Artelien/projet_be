use crate::trait_structure::{self, structure_donnee};


#[derive (Clone, Debug)]
pub struct arbre{
    pub value: i32,
    pub left : Option<Box<arbre>>,
    pub right :  Option<Box<arbre>>
}


impl structure_donnee for arbre {
    fn new(x: i32) -> Self {
        Self { value: x, left: None, right: None }
    }

    fn add(&mut self, x : i32){
        if self.value > x {
            // boucle sur le cote gauche et rappelle la fonction si c'est pas une feuille
             match &mut self.left {
                Some(g) => g.add(x),
                None => self.left = Some(Box::new(arbre::new(x))),
             }
        } else {
            
            // boucle sur le cote droit et rappelle la fonction si c'est pas une feuille
             match &mut self.right {
                Some(r) => r.add(x),
                None => self.right = Some(Box::new(arbre::new(x))),
             }
        }
    }
    
    // a verifier l utilite
    fn remove(&mut self, x : i32) {
        todo!()
    }
    
    fn parcourir(&self) {
        todo!()
    }

    fn fragmenter(&self, taille_max : usize) -> Vec<Self> where Self: Sized + Clone {
        todo!()
    }
    
    fn serialiser(&self) {
        todo!()
    }
    
    fn deserialiser(&self) {
        todo!()
    }


}