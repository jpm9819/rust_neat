#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{ Index, IndexMut };
use std::slice::{ Iter, IterMut };

pub struct HashVec<K, T>
where
    K: Eq + PartialEq + Hash,
    T: PartialOrd + Ord
{
    set: HashMap<K, usize>,
    data: Vec<T>,
}

impl<K, T> HashVec<K, T>
where
    K: Eq + PartialEq + Hash,
    T: PartialOrd + Ord
{
    pub fn new() -> HashVec<K, T> {
        HashVec{
            set: HashMap::new(),
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, item: T) -> usize {
        match self.set.get(&key) {
            Some(&index) => {
                if self.data[index] != item {
                    self.data[index] = item;
                }
                index
            },
            None => {
                self.data.push(item);
                let index = self.data.len() - 1;
                self.set.insert(key, index);
                index
            }
        }
    }

    pub fn insert_ordered(&mut self, key: K, item: T) -> usize {
        if !self.data.is_sorted() {
            self.data.sort();
        }
        let index = match self.set.get(&key) {
            Some(&index) => {
                if self.data[index] != item {
                    self.data[index] = item
                }
                index
            },
            None => {
                let mut index = self.data.len();
                for (i, it2) in self.data.iter().enumerate() {
                    match item.cmp(it2) {
                        Ordering::Less => {
                            index = i;
                            break;
                        },
                        _ => continue
                    }
                }
                self.data.insert(index, item);
                index
            }
        };
        index
    }

    pub fn contains(&self, key: K) -> bool {
        self.set.contains_key(&key)
    }

    pub fn get(&self, key: K) -> Option<&T> {
        let index = match self.set.get(&key) {
            Some(&index) => index,
            None => return None
        };

        Some(&self.data[index])
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut T> {
        let index = match self.set.get(&key) {
            Some(&index) => index,
            None => return None
        };

        Some(self.data.index_mut(index))
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        if index < self.data.len() {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.data.len() {
            Some(self.data.index_mut(index))
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }
}

impl<K, T> Index<usize> for HashVec<K, T>
where
    K: Eq + PartialEq + Hash,
    T: PartialOrd + Ord
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<K, T> IndexMut<usize> for HashVec<K, T>
where
    K: Eq + PartialEq + Hash,
    T: PartialOrd + Ord
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut hv: HashVec<usize, usize> = HashVec::new();
        for i in 0..5 {
            hv.insert(i, i+1);
        }

        for (i, val) in hv.iter().enumerate() {
            assert_eq!(i, val - 1);
        }
    }

    #[test]
    fn test_get() {
        let mut hv: HashVec<usize, usize> = HashVec::new();
        
        let keys = &[1, 2, 3];
        let values = &[7, 8, 9];

        for (&key, &value) in keys.iter().zip(values.iter()) {
            hv.insert(key, value);
        }

        assert_eq!(*hv.get(2).unwrap(), 8);

        assert_eq!(hv.get(5), None);
    }

    #[test]
    fn test_insert_ordered() {
        let mut hv: HashVec<usize, usize> = HashVec::new();

        let values = &[9, 8, 7];

        for (i, &val) in values.iter().enumerate() {
            hv.insert_ordered(i, val);
        }

        let values = &[7, 8, 9];

        for (&i1, &i2) in hv.iter().zip(values.iter()) {
            assert_eq!(i1, i2);
        }
    }
}