use std::{borrow::Borrow, hash::Hash};

use fxhash::FxHashMap;

pub trait DimensionalMap<K, V, const N: usize = 2>:
    FromIterator<([K; N], V)> + MinMax<K, N>
{
    fn insert<U: Into<V>>(&mut self, k: [K; N], v: U);
    fn get<U: Borrow<[K; N]>>(&self, k: &U) -> Option<&V>;
}

//------------------------------------------
// DimensionalHashMap
//------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct DimensionalHashMap<K: Copy, V, const N: usize = 2> {
    map: FxHashMap<[K; N], V>,
    min_max: MinMaxHolder<K, N>,
}

impl<K, V, const N: usize> MinMax<K, N> for DimensionalHashMap<K, V, N>
where
    K: Copy + Clone + Ord + Default,
{
    fn min(&self) -> &[Option<K>; N] {
        self.min_max.min()
    }

    fn max(&self) -> &[Option<K>; N] {
        self.min_max.max()
    }
}

impl<K, V, const N: usize> DimensionalMap<K, V, N> for DimensionalHashMap<K, V, N>
where
    K: Copy + Clone + Ord + Default + Hash,
    V: Default,
{
    fn insert<U: Into<V>>(&mut self, k: [K; N], v: U) {
        self.min_max.insert(k);
        self.map.insert(k, v.into());
    }

    fn get<U: Borrow<[K; N]>>(&self, k: &U) -> Option<&V> {
        self.map.get(k.borrow())
    }
}

impl<K, V, const N: usize> FromIterator<([K; N], V)> for DimensionalHashMap<K, V, N>
where
    K: Default + Copy + Clone + Ord + Hash,
    V: Default,
{
    fn from_iter<I: IntoIterator<Item = ([K; N], V)>>(iter: I) -> Self {
        let mut this = Self::default();
        for (k, v) in iter {
            this.insert(k, v);
        }
        this
    }
}

//------------------------------------------
// DimensionalVecMap
//------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct Dim2VecMap<K: Copy, V> {
    map: Vec<Vec<V>>,
    min_max: MinMaxHolder<K, 2>,
}

impl<K, V> MinMax<K, 2> for Dim2VecMap<K, V>
where
    K: Copy + Clone + Ord + Default,
{
    fn min(&self) -> &[Option<K>; 2] {
        self.min_max.min()
    }

    fn max(&self) -> &[Option<K>; 2] {
        self.min_max.max()
    }
}

impl<K, V> DimensionalMap<K, V, 2> for Dim2VecMap<K, V>
where
    K: Copy + Clone + Ord + Default + Into<isize>,
    V: Default,
{
    fn insert<U: Into<V>>(&mut self, k: [K; 2], v: U) {
        self.min_max.insert(k);
        let d1: isize = k[0].into();
        if d1 < 0 {
            return;
        }
        let d2: isize = k[1].into();
        if d2 < 0 {
            return;
        }

        if self.map.get(d1 as usize).is_none() {
            self.map[d1 as usize] = Vec::with_capacity(d2 as usize);
        };
        self.map[d1 as usize][d2 as usize] = v.into();
    }

    fn get<U: Borrow<[K; 2]>>(&self, k: &U) -> Option<&V> {
        let k = k.borrow();
        let d1: isize = k[0].into();
        if d1 < 0 {
            return None;
        }
        let d2: isize = k[1].into();
        if d2 < 0 {
            return None;
        }
        self.map[d1 as usize].get(d2 as usize)
    }
}

impl<K, V> FromIterator<([K; 2], V)> for Dim2VecMap<K, V>
where
    K: Default + Copy + Clone + Ord + Into<isize>,
    V: Default,
{
    fn from_iter<I: IntoIterator<Item = ([K; 2], V)>>(iter: I) -> Self {
        let mut this = Self::default();
        for (k, v) in iter {
            this.insert(k, v);
        }
        this
    }
}

//------------------------------------------
// MinMax
//------------------------------------------

pub trait MinMax<T, const N: usize> {
    fn min(&self) -> &[Option<T>; N];

    fn max(&self) -> &[Option<T>; N];
}

#[derive(Debug, Clone)]
struct MinMaxHolder<T, const N: usize> {
    min: [Option<T>; N],
    max: [Option<T>; N],
}

impl<T: Clone + Ord, const N: usize> MinMaxHolder<T, N> {
    fn insert(&mut self, value: [T; N]) {
        for (idx, t) in value.iter().enumerate() {
            if let Some(current) = self.min.get(idx).unwrap() {
                if t < current {
                    self.min[idx] = Some(t.clone());
                }
            } else {
                self.min[idx] = Some(t.clone());
            }

            if let Some(current) = self.max.get(idx).unwrap() {
                if t > current {
                    self.max[idx] = Some(t.clone());
                }
            } else {
                self.max[idx] = Some(t.clone());
            }
        }
    }
}

impl<T: Copy + Default, const N: usize> Default for MinMaxHolder<T, N> {
    fn default() -> Self {
        Self {
            min: [Default::default(); N],
            max: [Default::default(); N],
        }
    }
}

impl<T: Clone + Ord, const N: usize> MinMax<T, N> for MinMaxHolder<T, N> {
    fn min(&self) -> &[Option<T>; N] {
        &self.min
    }

    fn max(&self) -> &[Option<T>; N] {
        &self.max
    }
}
