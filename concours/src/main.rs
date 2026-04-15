mod arbre;
mod trait_structure;

use crate::{arbre::Arbre, trait_structure::structure_donnee};

fn main() {
    /*
     * Test des arbres
     *
     *
     */
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

    // test map
    test.map(|x| {
        print!("{} ", x);
        x
    });

    // test iterateur
    let mut iter = test.iter();
    println!();
    while let Some(value) = iter.next() {
        print!("{} ", value);
    }
    let mut tab = test.fragmenter(2);

    // test de fragmentation
    print!("\nFragmentation : \n");
    for elem in &tab {
        print!("    Nouvelle arbe : ");
        elem.map(|x| {
            print!("{} ", x);
            x
        });
        println!();
    }

    // utilisation de split et union avec un vec
    let (gauche, droite) = tab.split_at_mut(1);
    let a = &mut gauche[0];
    let b = &droite[0];
    a.union(b);
    a.map(|x| {
        print!("{} ", x);
        x
    });
}
