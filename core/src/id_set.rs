use num_traits::sign::Unsigned;
use optional::Noned;
use optional::Optioned;
use std::default::Default;
use std::marker::PhantomData;

pub trait IdIndex: Unsigned + Into<usize> + From<usize> + Copy {}

pub trait IdGeneration: Unsigned + Into<usize> + From<usize> + Copy {}

pub trait Id<TIndex: IdIndex, TGeneration: IdGeneration>: Noned + Clone + Copy + Sized {
    fn new(index: TIndex, generation: TGeneration) -> Self;

    fn get_index(&self) -> TIndex;

    fn get_generation(&self) -> TGeneration;

    fn null() -> Self {
        Self::new(TIndex::zero(), TGeneration::zero())
    }
}

pub struct IdSet<T, TIndex, TGeneration, TId>
where
    T: Default,
    TIndex: IdIndex,
    TGeneration: IdGeneration,
    TId: Id<TIndex, TGeneration>,
{
    _generations: Vec<TGeneration>,
    _free_indicies: Vec<TIndex>,
    _filled_indicies_count: usize,
    _values: Vec<T>,
    phantom: PhantomData<TId>,
}

impl<T, TIndex, TGeneration, TId> IdSet<T, TIndex, TGeneration, TId>
where
    T: Default,
    TIndex: IdIndex,
    TGeneration: IdGeneration,
    TId: Id<TIndex, TGeneration>,
{
    pub fn new() -> Self {
        IdSet {
            _generations: Vec::new(),
            _free_indicies: Vec::new(),
            _filled_indicies_count: 0,
            _values: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self._generations.len() - self._free_indicies.len() - self._filled_indicies_count
    }

    pub fn exists(&self, id: TId) -> bool {
        let index = id.get_index().into();
        index < self._generations.len() && self._generations[index] == id.get_generation()
    }

    pub fn add(&mut self, value: T) -> TId {
        let index = if self._free_indicies.is_empty() {
            self._generations.len()
        } else {
            self._free_indicies.pop().unwrap().into()
        };

        // assert!(index != TIndex::MAX);

        if index == self._generations.len() {
            self._generations.push(TGeneration::zero());
            self._values.push(value);
            TId::new(index.into(), TGeneration::zero())
        } else {
            self._values[index] = value;
            TId::new(index.into(), TGeneration::zero())
        }
    }

    pub fn remove(&mut self, id: TId) {
        assert!(!self.exists(id));

        self._free_indicies.push(id.get_index());
        let index = id.get_index().into();
        self._generations[index] = self._generations[index] + TGeneration::one();
        self._values[index] = T::default();
    }

    pub fn clear(&mut self) {
        panic!("not implemented")
    }

    pub fn clear_all(&mut self) {
        self._generations.clear();
        self._free_indicies.clear();
        self._values.clear();
        self._filled_indicies_count = 0;
    }

    pub fn get(&self, _id: TId) -> &T {
        panic!("not implemented")
    }

    pub fn set(&mut self, _id: TId, _value: T) {
        panic!("not implemented")
    }

    pub fn find() -> Optioned<TId> {
        panic!("not implemented")
    }

    pub fn find_all() -> Vec<TId> {
        panic!("not implemented")
    }

    pub fn optimize() {
        panic!("not implemented")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
