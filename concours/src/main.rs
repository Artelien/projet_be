mod tree;
mod trait_structure;

use crate::{tree::Arbre, trait_structure::StructureDonnee};
use rand::Rng;

fn main() {
    /*
     * Test des arbres
     *
     *
     */
    let mut test = Arbre::new();
    let mut rng = rand::thread_rng();
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..10 {
        vec.push(rng.gen_range(0..500));
    }

    for elem in &vec {
        test.add(*elem);
    }

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
    let b = &mut droite[0];
    a.union(b);
    a.map(|x| {
        print!("{} ", x);
        x
    });
    println!();
    b.map(|x| {
        print!("{} ", x);
        x
    });
    println!("\nDifference sysmetrique ");

    a.difference_symetrique(b);
    a.map(|x| {
        print!("{} ", x);
        x
    });

    println!();
    b.map(|x| {
        print!("{} ", x);
        x
    });

    // let c = a.intersection(b);
    let mut new = Arbre::new();
    for elem in &tab {
        let mut iter = elem.iter();
        while let Some(value) = iter.next(){
            new.add(*value);
        }
    }

    println!("New : {:?}", new);
    new.remove(*vec.get(2).unwrap());
    println!("Valeur a enlever : {}", *vec.get(2).unwrap());
    println!("New : {:?}", new);


    new.map(|x| {
        print!("{} ", x);
        x
    });

    println!();
    new.difference(&mut tab[0]);
    new.map(|x| {
        print!("{} ", x);
        x
    });
    new.intersection(&mut tab[4]);
    println!("\n{:?}", tab[4]);
    new.map(|x| {
        print!("{} ", x);
        x
    });

    drop(new);
}
