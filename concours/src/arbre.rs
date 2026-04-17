use std::{cmp::Ordering, ops::{Deref, DerefMut}};

use crate::trait_structure::StructureDonnee;

#[derive(Clone, Debug)]
pub struct Arbre {
    root: Option<Node>,
    size: usize,
}

#[derive(Clone, Debug)]
struct Node {
    value: i32,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl Node {
    fn new(x: i32) -> Self {
        Self {
            value: x,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, x: i32) {
        if self.value > x {
            // boucle sur le cote gauche et rappelle la fonction si c'est pas une feuille
            match &mut self.left {
                Some(g) => g.add(x),
                None => self.left = Some(Box::new(Node::new(x))),
            }
        } else if self.value < x {
            // boucle sur le cote droit et rappelle la fonction si c'est pas une feuille
            match &mut self.right {
                Some(r) => r.add(x),
                None => self.right = Some(Box::new(Node::new(x))),
            }
        }
    }

    fn map(&self, f: impl Fn(i32) -> i32 + Copy) -> Node {
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


    fn remove(&mut self, value: i32){
        if value == self.value {
            if let Some(remplacement) = Node::delete_node(Some(Box::new(Node {
                value: self.value,
                left: self.left.take(),
                right: self.right.take(),
            }))) {
                self.value = remplacement.value;
                self.left  = remplacement.left;
                self.right = remplacement.right;
            } 
        } else if value < self.value {
            if let Some(l) = self.left.as_mut() {
                if l.value == value && l.is_leaf() {
                    self.left = None;
                } else {
                    l.remove(value);
                }
            } 
        } else {
            if let Some(r) = self.right.as_mut(){
                if r.value == value && r.is_leaf() {
                    self.right = None;
                } else {
                    r.remove(value);
                }
            }
        } 
    }


    fn successeur(&self) -> &Self{
        let mut succ = self.right.as_ref().unwrap();
        while let Some(l) = succ.left.as_ref() {
            succ = l;
        }
        succ
    }


    fn delete_node(noeud: Option<Box<Node>>) -> Option<Box<Node>> {
        match noeud {
            None => None,

            Some(n) if n.left.is_none() && n.right.is_none() => {
                None // cas 1 : feuille — on supprime directement
            }

            Some(n) if n.left.is_none() => {
                n.right // cas 2a : un seul enfant à droite
            }

            Some(n) if n.right.is_none() => {
                n.left // cas 2b : un seul enfant à gauche
            }

            Some(mut n) => {
                // cas 3 : deux enfants
                // on remplace par le plus petit de droite (successeur infixe)
                let succ_value = n.right.as_ref().unwrap().successeur().value;
                n.value = succ_value;
                // on supprime le successeur du sous-arbre droit
                n.right = Node::delete_node(n.right.take());
                Some(n)
            }
        }

    }

    fn is_leaf(&self) -> bool{
        self.left.is_none() && self.right.is_none()
    }
}

pub struct IterateurArbre<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for IterateurArbre<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        if let Some(left) = &node.left {
            self.stack.push(left);
        }
        if let Some(right) = &node.right {
            self.stack.push(right);
        }
        Some(&node.value)
    }
}

impl Arbre {
    pub fn iter(&self) -> IterateurArbre<'_> {
        let mut stack = Vec::new();
        if let Some(root) = &self.root {
            stack.push(root);
        }
        IterateurArbre { stack }
    }
}

impl StructureDonnee for Arbre {
    fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    fn add(&mut self, value: i32) {
        match &mut self.root {
            Some(node) => node.add(value),
            None => self.root = Some(Node::new(value)),
        }
        self.size += 1;
    }

    fn there_is(&self, value: i32) -> bool {
        let mut current = self.root.as_ref();
        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => current = node.left.as_deref(),
                Ordering::Greater => current = node.right.as_deref(),
            }
        }
        false
    }

    fn remove(&mut self, value: i32) {
        if let Some(node) = self.root.as_mut() {
            if node.value != value || !node.is_leaf() {
                node.remove(value);
            } else {
                self.root = None;
            }
        }
        self.size -= 1;
    }

    fn map(&self, f: impl Fn(i32) -> i32 + Copy) -> Arbre {
        Arbre {
            size: self.size,
            root: self.root.as_ref().map(|node| node.map(f)),
        }
    }

    fn fragmenter(self, taille_max: usize) -> Vec<Self>
    where
        Self: Sized + Clone,
    {
        let mut result = Vec::new();
        let mut buffer: Vec<Node> = Vec::new();

        if let Some(n) = self.root {
            n.traverse(&mut buffer);
        }

        let mut nb = 0;
        let mut current = Arbre::new();

        for elem in buffer {
            current.add(elem.value);
            nb += 1;
            if nb % taille_max == 0 {
                result.push(current);
                current = Arbre::new();
            }
        }
        if nb % taille_max != 0 {
            result.push(current);
        }
        result
    }

    fn serialiser(&self) {
        todo!()
    }

    fn deserialiser(&self) {
        todo!()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &i32> + '_> {
        Box::new(self.iter())
    }
}
