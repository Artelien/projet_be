#set page(
  paper: "a4",
  margin: (x: 1.5cm, y: 2cm),
  columns: 2,
)

#set text(font: "Linux Libertine", size: 10pt, lang: "fr")
#set par(justify: true, leading: 0.65em)
#set heading(numbering: "1.")

#show heading.where(level: 1): it => {
  set text(size: 11pt, weight: "bold")
  block(above: 1.2em, below: 0.6em, it)
}

#show heading.where(level: 2): it => {
  set text(size: 10pt, weight: "bold")
  block(above: 0.8em, below: 0.4em, it)
}

#place(
  top + center,
  scope: "parent",
  float: true,
  block(width: 100%, {
    align(center)[
      #text(size: 16pt, weight: "bold")[Bilan mi-parcours — Structures de données en Rust]
      #v(0.3em)
      #text(size: 10pt, style: "italic")[Projet BE — Phase 1]
      #v(0.3em)
      #text(size: 10pt)[Maximilien Larroche 22301310]
      #v(0.3em)
      #line(length: 100%, stroke: 0.5pt)
      #v(0.5em)
    ]
  }),
)

= Introduction

Ce rapport présente le bilan de la première phase du projet, dont l'objectif est d'implémenter plusieurs structures de données en Rust en respectant un trait commun, puis d'évaluer leurs performances. Cette phase s'est concentrée sur l'apprentissage du langage Rust et l'implémentation de trois structures : l'arbre binaire de recherche (BST), un HashSet et un BTreeSet.

= Implémentation

== Apprentissage de Rust

L'apprentissage du langage Rust a constitué la partie la plus chronophage de cette phase. Les ressources utilisées sont le Rust Book #link(label("ref1"))[[1]] ainsi que des vidéos en français #link(label("ref2"))[[2]]#link(label("ref3"))[[3]]#link(label("ref4"))[[4]]. Les principales difficultés rencontrées concernent le système d'ownership, les types mutables, et la gestion des références.

== Trait commun

Un trait `StructureDonnee` a été défini pour unifier les opérations de base sur toute structure de données :

```rust
pub trait StructureDonnee {
    fn new() -> Self;
    fn add(&mut self, value: i32);
    fn remove(&mut self, value: i32);
    fn there_is(&self, value: i32) -> bool;
    fn map(&self, f: impl Fn(i32) -> i32 + Copy) -> Self;
    fn iter(&self) -> Box<dyn Iterator<Item = &i32> + '_>;
    fn fragmenter(self, taille_max: usize) -> Vec<Self>
    where
        Self: Sized + Clone;
}
```

Les opérations ensemblistes (union, intersection, différence, différence symétrique) ont été implémentées directement dans le trait à partir de `add`, `remove`, `iter` et `there_is`. Ce choix évite de les ré-implémenter pour chaque structure et permet d'ajouter facilement de nouvelles structures.

La fonction `delete` initialement prévue a été supprimée car Rust implémente le trait `Drop` qui libère la mémoire automatiquement à la sortie du scope.

== Arbre binaire de recherche

L'arbre binaire de recherche a été choisi comme première structure car il est bien maîtrisé. La structure est définie ainsi :

#colbreak()
```rust
pub struct Arbre {
    pub taille: usize,
    pub root: Option<Node>,
}
pub struct Node {
    pub value: i32,
    pub left:  Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}
```

L'utilisation de `Box<Node>` est nécessaire car Rust doit connaître la taille de chaque type à la compilation. Un type récursif direct aurait une taille infinie. `Option` permet de représenter l'absence d'enfant (`None`).

La suppression d'un nœud gère trois cas : nœud feuille (suppression directe), nœud avec un seul enfant (remplacement par l'enfant), et nœud avec deux enfants (remplacement par le successeur infixe — le nœud le plus à gauche du sous-arbre droit).

Un itérateur en profondeur (DFS) a été implémenté via une pile explicite pour parcourir l'arbre sans récursion, évitant les débordements de pile sur les grands arbres.

== HashSet et BTreeSet

Pour permettre la comparaison, deux structures de la bibliothèque standard ont été wrappées via le pattern _newtype_ afin de respecter la règle orpheline de Rust — qui interdit d'implémenter un trait extérieur sur un type extérieur :

```rust
pub struct MonHashSet(pub HashSet<i32>);
pub struct MonBTree(pub BTreeSet<i32>);
```

Le `HashSet` utilise une table de hachage offrant des opérations en O(1) en moyenne, mais sans ordre garanti. Le `BTreeSet` est un arbre B équilibré de la bibliothèque standard offrant des opérations en O(log n) avec un ordre trié garanti, comparable à un BST optimisé.

== Opérations ensemblistes

Les quatre opérations ont été implémentées dans le trait en modifiant la structure appelante (`&mut self`) plutôt qu'en créant une nouvelle structure, afin de minimiser les allocations mémoire :

- *Union* : insertion de tous les éléments de `other` dans `self`
- *Intersection* : suppression des éléments de `self` absents de `other`
- *Différence* : suppression des éléments de `self` présents dans `other`
- *Différence symétrique* : collecte d'abord les éléments communs, puis suppression dans les deux structures pour éviter les conflits d'emprunt du borrow checker

#pagebreak()
= Vérification fonctionnelle

La vérification s'est faite par tests manuels dans le `main` avec `println!`, en vérifiant le comportement dans les cas limites : arbre vide, nœud feuille, nœud avec un ou deux enfants, suppression de la racine.

Des fonctions de test unitaires avec `#[test]` n'ont pas été implémentées par manque de temps. Elles sont prévues pour la phase suivante et permettront à toute nouvelle structure implémentant le trait de passer les mêmes tests automatiquement.

= Protocole expérimental

Les benchmarks ont été réalisés avec la crate Criterion #link(label("ref5"))[[5]] (version 0.5), qui effectue des mesures statistiques avec intervalle de confiance à 95%.

*Environnement* : Windows 11, processeur x86\_64, avec uniquement VS Code ouvert pendant les mesures.

*Paramètres* : 50 échantillons par benchmark, 15 secondes de mesure, phase de chauffe de 3 secondes (valeur par défaut Criterion). Les structures sont remplies avec des valeurs aléatoires (crate `rand`) pour éviter la dégénérescence de l'arbre en liste chaînée lors des insertions ordonnées.

Les benchmarks ont été généralisés via une fonction générique contrainte par le trait, ce qui permet de comparer toutes les structures sans dupliquer le code :

```rust
fn bench_generique<T: StructureDonnee>(
    c: &mut Criterion, nom: &str
) { ... }
```

= Résultats

Les benchmarks ont été réalisés sur des structures de 10 000 éléments. Les temps indiqués sont les médianes des 50 mesures.

#figure(
  table(
    columns: (auto, auto, auto, auto),
    stroke: 0.5pt,
    align: (left, right, right, right),
    [*Opération*], [*BST*], [*HashSet*], [*BTreeSet*],
    [add 10 000], [1.05 ms], [287 µs], [284 µs],
    [remove 10 000], [1.35 ms], [413 µs], [413 µs],
    [find], [9.0 ns], [7.3 ns], [7.3 ns],
    [Union], [1.14 ms], [145 µs], [143 µs],
    [Différence], [1.35 ms], [269 µs], [270 µs],
    [Diff. symétrique], [33.4 ns], [39.0 ns], [39.6 ns],
    [Intersection], [1.14 ms], [145 µs], [147 µs],
    [Fragmenter], [2.30 ms], [109 µs], [109 µs],
  ),
  caption: [Résultats des benchmarks — 10 000 éléments],
)

Les résultats montrent que le HashSet et le BTreeSet sont significativement plus rapides que le BST maison sur la quasi-totalité des opérations — de 3x à 20x selon l'opération. Cela s'explique par les optimisations de la bibliothèque standard (cache-friendly, équilibrage automatique) et par le fait que le BST maison n'est pas équilibré.

La différence symétrique fait exception : le BST est plus rapide (33 ns vs 39 ns) car l'implémentation profite de la propriété d'ordre du BST pour collecter les éléments communs plus efficacement.

La recherche (`find`) est proche pour les trois structures (~7-9 ns), ce qui confirme la complexité O(log n) théorique du BST et du BTreeSet, et O(1) du HashSet.

Le BST maison souffre probablement d'un déséquilibre résiduel malgré l'insertion aléatoire, ce qui dégrade ses performances par rapport au BTreeSet de la bibliothèque standard qui garantit l'équilibrage.

= Conclusion

Cette première phase a permis de poser les bases du projet : maîtrise du langage Rust, définition d'un trait commun, implémentation complète d'un BST avec les opérations ensemblistes, et comparaison avec deux structures de la bibliothèque standard (HashSet et BTreeSet).

Les résultats montrent clairement l'impact des optimisations de la bibliothèque standard sur les performances. Pour la phase suivante, les priorités sont : l'implémentation de tests unitaires automatisés, l'ajout d'un arbre AVL pour comparer avec le BST maison, et la sérialisation/désérialisation.

= Références

#set text(size: 9pt)
#set par(justify: false)

[1] #label("ref1") Klabnik S., Nichols C. _The Rust Programming Language_, 2023. #link("https://doc.rust-lang.org/book/")

[2] #label("ref2") _Apprendre le RUST partie \#1 FR_, YouTube. #link("https://www.youtube.com/watch?v=mZasv3__A9k&list=PLrT8DrHsxZTiiAj96QukmAdedfRMsIPN5")

[3] #label("ref3") _Apprendre le RUST partie \#2 FR_, YouTube. #link("https://www.youtube.com/watch?v=wgjw5lGv-EI&list=PLrT8DrHsxZTiiAj96QukmAdedfRMsIPN5&index=2")

[4] #label("ref4") _Apprendre le RUST partie \#3 FR_, YouTube. #link("https://www.youtube.com/watch?v=3kBk3sjREOM&list=PLrT8DrHsxZTiiAj96QukmAdedfRMsIPN5&index=3")

[5] #label("ref5") Heisler B. _Criterion.rs — Statistics-driven benchmarking_, 2024. #link("https://bheisler.github.io/criterion.rs/book/")

[6] Dépôt source : #link("https://github.com/Artelien/projet_be")
