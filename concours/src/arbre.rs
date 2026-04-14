use std::vec;

use crate::trait_structure::{self, structure_donnee};


#[derive (Clone, Debug)]
pub struct Arbre{
    root : Option<Node>,
    size : usize,
}

#[derive (Clone, Debug)]
struct Node{
    value: i32,
    left : Option<Box<Self>>,
    right :  Option<Box<Self>>
}

impl Node {
    fn new(x: i32) -> Self{
        Self{value: x, left: None, right: None}
    }


    fn add(&mut self, x : i32){
        if self.value > x {
            // boucle sur le cote gauche et rappelle la fonction si c'est pas une feuille
             match &mut self.left {
                Some(g) => g.add(x),
                None => self.left = Some(Box::new(Node::new(x))),
             }
        } else {
            
            // boucle sur le cote droit et rappelle la fonction si c'est pas une feuille
             match &mut self.right {
                Some(r) => r.add(x),
                None => self.right = Some(Box::new(Node::new(x))),
             }
        }
    }


    fn map(&self,  f: impl Fn(i32) -> i32 + Copy) -> Node {
        Node { 
            value: f(self.value),
            left: self.left.as_ref().map(|l| Box::new(l.map(f))), 
            right: self.right.as_ref().map(|r| Box::new(r.map(f))),
        }
    }

    fn traverse(mut self, buffer: &mut Vec<Node>) {
        if let Some(left) = self.left.take() {
            left.traverse(buffer);
        }
        if let Some(right) = self.right.take() {
            right.traverse(buffer);
        }
        buffer.push(self);
    }


}


impl structure_donnee for Arbre {
    fn new() -> Self {
        Self { root: None, size : 1}
    }

    fn add(&mut self, x : i32){
        match &mut self.root {
            Some(node) => node.add(x),
            None => self.root = Some(Node::new(x)),
        }
        self.size += 1;
    }
    
    // a verifier l utilite
    fn remove(&mut self, x : i32) {
        todo!()
    }
    
    fn map(&self,  f: impl Fn(i32) -> i32 + Copy) -> Arbre {
        Arbre { 
            size: self.size,
            root: self.root.as_ref().map(|node| node.map(f)),
        }
    }

    fn fragmenter(self, taille_max : usize) -> Vec<Self> where Self: Sized + Clone {
        let mut result = Vec::new();
        let mut buffer: Vec<Node>  = Vec::new();

        if let Some(n) = self.root{
            n.traverse(&mut buffer);
        }

        let mut nb = 0;
        let mut current = Arbre::new(); 

        for elem in buffer{
                nb += 1;
                if nb % taille_max == 0 {
                    result.push(current);
                    current = Arbre::new();
                }
                current.add(elem.value);
        }
        
        result
    }
    
    fn serialiser(&self) {
        todo!()
    }
    
    fn deserialiser(&self) {
        todo!()
    }


}