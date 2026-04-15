mod arbre;
mod trait_structure;

use crate::{arbre::Arbre, trait_structure::structure_donnee};

fn main() {
    let mut test = Arbre::new();
    test.add(1);
    test.add(2);
    test.add(3);
    test.add(4);
    test.add(5);
    test.add(6);
    test.add(7);
    test.add(7);
    test.add(8);
    test.map(|x| {
        print!("{} ", x);
        x
    });

    let mut iter = test.iter();

    println!();
    while let Some(value) = iter.next() {
        print!("{} ", value);
    }
    let mut tab = test.fragmenter(2);

    print!("\nFragmentation : \n");

    for elem in tab {
        print!("    Nouvelle arbe : ");
        elem.map(|x| {
            print!("{} ", x);
            x
        });
        println!();
    }
}
