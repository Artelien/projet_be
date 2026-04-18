use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkGroup, measurement::WallTime};
use concours::{Arbre, StructureDonnee, tree};
use rand::{seq::SliceRandom, thread_rng};


fn structure_full<T: StructureDonnee>(n: i32) -> T {
    let mut s = T::new();
    let mut valeurs: Vec<i32> = (0..n).collect();
    valeurs.shuffle(&mut thread_rng()); // mélange aléatoire
    for v in valeurs { s.add(v); }
    s
}

fn bench_two_structures<T>(group: &mut BenchmarkGroup<WallTime>, name: &str, f: impl Fn(&mut T, &T), number: i32)
where
    T: StructureDonnee,
{
    group.bench_function(name, |b| {
        b.iter_batched(
            || (structure_full::<T>(number), structure_full::<T>(number)),
            |(mut s1, s2)| f(&mut s1, black_box(&s2)),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_generique<T>(c: &mut Criterion, name: &str, number: i32)
    where
    T: StructureDonnee + Clone,
{
    let mut structure = c.benchmark_group(name);
    
structure.bench_function(format!("add {} noeuds", number), |b| {
    b.iter(|| {
        let mut s = T::new();
        let mut valeurs: Vec<i32> = (0..number).collect();
        valeurs.shuffle(&mut rand::thread_rng());
        for i in valeurs {
            s.add(black_box(i));
        }
    })
});

    structure.bench_function(format!("remove {} noeuds", number) , |b| {
        b.iter(|| {
            let mut s: T = structure_full(number);
            for i in 0..number { s.remove(black_box(i)); }
        })
    });


    structure.bench_function("find", |b| {
        let s : T = structure_full(number);
        b.iter(|| s.there_is(black_box(42)))
    });


    bench_two_structures::<T>(&mut structure, "Union", |s1, s2| s1.union(s2), number);

    // structure.bench_function("Union", |b| {
    //     let mut s1: T = structure_full(500);
    //     let s2: T = structure_full(500);
    //     b.iter(|| s1.union(&s2))
    // });

    bench_two_structures::<T>(&mut structure, "Difference", |s1, s2| s1.difference(s2), number);

    // structure.bench_function("Difference", |b| {
    //     let mut s1: T = structure_full(number);
    //     let s2: T = structure_full(number);
    //     b.iter(|| s1.difference(&s2))
    // });


    structure.bench_function("Difference sysmétrique", |b| {
        let mut s1: T = structure_full(500);
        let mut s2: T = structure_full(500);
        b.iter(|| s1.difference_symetrique(&mut s2))
    });

    bench_two_structures::<T>(&mut structure, "Intersection", |s1, s2| {s1.intersection(s2);}, number);


    // structure.bench_function("Intersection", |b| {
    //     let s1: T = structure_full(500);
    //     let s2: T = structure_full(500);
    //     b.iter(|| s1.intersection(&s2))
    // });

    structure.bench_function("Fragmenter", |b| {
        let s1: T = structure_full(number);
        b.iter(|| s1.clone().fragmenter((number as usize)/20))
    });

    structure.finish();
}



fn bench_tree(c: &mut Criterion){
    bench_generique::<Arbre>(c, "Tree", 10000);
}

criterion_group!{name = benches; config=Criterion::default().measurement_time(Duration::from_secs(15)).sample_size(50); targets = bench_tree}
criterion_main!(benches);