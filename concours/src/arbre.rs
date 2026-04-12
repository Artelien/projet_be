use crate::trait_structure::{self, structure_donnee};


#[derive (Clone, Debug)]
pub struct Arbre{
    pub value: i32,
    pub left : Option<Box<Arbre>>,
    pub right :  Option<Box<Arbre>>
}


impl structure_donnee for Arbre {
    fn new(x: i32) -> Self {
        Self { value: x, left: None, right: None }
    }

    fn add(&mut self, x : i32){
        if self.value > x {
            // boucle sur le cote gauche et rappelle la fonction si c'est pas une feuille
             match &mut self.left {
                Some(g) => g.add(x),
                None => self.left = Some(Box::new(Arbre::new(x))),
             }
        } else {
            
            // boucle sur le cote droit et rappelle la fonction si c'est pas une feuille
             match &mut self.right {
                Some(r) => r.add(x),
                None => self.right = Some(Box::new(Arbre::new(x))),
             }
        }
    }
    
    // a verifier l utilite
    fn remove(&mut self, x : i32) {
        todo!()
    }
    
    fn map(&self,  f: impl Fn(i32) -> i32 + Copy) -> Arbre {
        Arbre { 
            value: f(self.value), 
            left: self.left.as_ref().map(|l| Box::new(l.map(f))), 
            right: self.right.as_ref().map(|r| Box::new(r.map(f))),
        }
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