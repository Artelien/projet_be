mod trait_structure;
mod arbre;


use crate::{arbre::Arbre, trait_structure::structure_donnee};


fn main() {
    let mut test = Arbre::new();
    test.add(1);
    test.add(2);
    test.add(3);
    print!("{:?}", test);
}
