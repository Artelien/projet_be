pub trait StructureDonnee {
    fn new() -> Self;

    fn add(&mut self, value: i32);

    fn remove(&mut self, value: i32);

    fn there_is(&self, value: i32) -> bool;

    fn map(&self, f: impl Fn(i32) -> i32 + Copy) -> Self;

    fn iter(&self) -> Box<dyn Iterator<Item = &i32> + '_>;

    // separer l'ensemble en liste de plus petit ensemble
    fn fragmenter(self, taille_max: usize) -> Vec<Self>
    where
        Self: Sized + Clone;

    fn intersection(&mut self, other: &Self)
    where
        Self: Sized,
    {
        let to_remove: Vec<i32> = self
            .iter()
            .filter(|&&v| !other.there_is(v))
            .copied()
            .collect();

        for v in to_remove {
            self.remove(v);
        }
    }

    fn union(&mut self, other: &Self)
    where
        Self: Sized,
    {
        let mut iter = other.iter();
        while let Some(value) = iter.next() {
            self.add(*value);
        }
    }

    fn difference(&mut self, other: &Self)
    where
        Self: Sized,
    {
        let mut iter = other.iter();
        while let Some(value) = iter.next() {
            if self.there_is(*value) {
                self.remove(*value);
            }
        }
    }

    fn difference_symetrique(&mut self, other: &mut Self)
    where
        Self: Sized,
    {

        let communs : Vec<i32> = other.iter().filter(|&&value| self.there_is(value)).copied().collect();

        

        for x in communs{
            self.remove(x);
            other.remove(x);
        }
    }

    fn serialiser(&self);

    fn deserialiser(&self);
}
