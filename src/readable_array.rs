use crate::index::Index;
use std::sync::RwLockReadGuard;

pub struct ReadableArray<'lock, T> {
    vec: RwLockReadGuard<'lock, Vec<T>>,
    indices: RwLockReadGuard<'lock, Vec<Index>>,
}

impl<'lock, T> ReadableArray<'lock, T> {
    pub fn new(
        vec: RwLockReadGuard<'lock, Vec<T>>,
        indices: RwLockReadGuard<'lock, Vec<Index>>,
    ) -> Self {
        Self { vec, indices }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            array: &self,
            next: 0,
        }
    }
}

pub struct Iter<'array, T> {
    array: &'array ReadableArray<'array, T>,
    next: usize,
}

impl<'array, 'lock, T> Iter<'array, T> {
    fn new(array: &'array ReadableArray<'array, T>) -> Self {
        Self { array, next: 0 }
    }
}

impl<'array, 'lock, T> Iterator for Iter<'array, T> {
    type Item = &'array T;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.array.indices.get(self.next)?;
        let value = unsafe { self.array.vec.get_unchecked(*index as usize) };
        self.next += 1;
        Some(value)
    }
}
