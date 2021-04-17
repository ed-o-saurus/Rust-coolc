use std::cmp::Eq;
use std::hash::Hash;

use indexmap::{IndexMap, IndexSet};

// This module contains "Scoped" data structures.
// This is useful for hierarchical data structures in which
// searching the tree causes object to go into and out of scope.

pub struct ScopedIndexSet<T> {
    sets: Vec<IndexSet<T>>,
}

impl<T> ScopedIndexSet<T>
where
    T: Eq,
    T: Hash,
{
    pub fn new() -> ScopedIndexSet<T> {
        ScopedIndexSet { sets: Vec::new() }
    }

    pub fn enter_scope(&mut self) {
        self.sets.push(IndexSet::new());
    }

    pub fn exit_scope(&mut self) {
        self.sets.pop();
    }

    // Search all scopes
    pub fn contains(&self, t: &T) -> bool {
        self.sets.iter().any(|hs| hs.contains(t))
    }

    // Only search current (top) scope
    pub fn contains_top_scope(&self, t: &T) -> bool {
        self.sets.last().unwrap().contains(t)
    }

    // Insert into the current (top) scope
    pub fn insert(&mut self, t: T) {
        self.sets.last_mut().unwrap().insert(t);
    }
}

pub struct ScopedIndexMap<K, V> {
    maps: Vec<IndexMap<K, V>>,
}

impl<K, V> ScopedIndexMap<K, V>
where
    K: Eq,
    K: Hash,
{
    pub fn new() -> ScopedIndexMap<K, V> {
        ScopedIndexMap { maps: Vec::new() }
    }

    pub fn enter_scope(&mut self) {
        self.maps.push(IndexMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.maps.pop();
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.maps.last_mut().unwrap().insert(k, v)
    }

    // Default to the highest (top) scope and work down from there.
    pub fn get(&self, k: &K) -> Option<&V> {
        for hm in self.maps.iter().rev() {
            if let Some(v) = hm.get(k) {
                return Some(v);
            }
        }

        None // Fall-through if k isn't found
    }

    // Convert to a regular (non-scoped) index map
    pub fn to_index_map(&self) -> IndexMap<K, V>
    where
        K: Clone,
        V: Clone,
    {
        let mut ret_val: IndexMap<K, V> = IndexMap::new();

        for map in self.maps.iter() {
            for (k, v) in map.iter() {
                ret_val.insert((*k).clone(), (*v).clone());
            }
        }

        ret_val
    }
}
