use core::{num::NonZero, sync::atomic::AtomicU64};
use std::{collections::BTreeMap, sync::RwLock};

use kiddo::{KdTree, SquaredEuclidean};

use crate::shader::{Shader, VertexDim};

#[derive(Debug)]
pub struct Memoize<const D: usize, F: VertexDim<D>, S: Shader<F>> {
    shader: S,
    cache: RwLock<BTreeMap<u64, S::Output>>,
    kd_tree: RwLock<KdTree<f64, D>>,
    threshold: Option<f64>,
    time_invalidates: bool,
    cached_time: AtomicU64,
    item_counter: AtomicU64,
}
impl<const D: usize, F: VertexDim<D>, S: Shader<F>> Memoize<D, F, S>
where
    S::Output: Clone,
{
    fn invalidate(&self) {
        self.cache.write().unwrap().clear();
        let mut kd_tree = self.kd_tree.write().unwrap();
        *kd_tree = KdTree::new();

        self.item_counter
            .store(0, core::sync::atomic::Ordering::Relaxed);
    }

    fn get(&self, key: [f64; D]) -> Option<S::Output> {
        let cache = self.cache.read().unwrap();
        let kd_tree = self.kd_tree.read().unwrap();

        let threshold = match self.threshold {
            Some(t) => t,
            None => f64::EPSILON,
        };
        let &nearest_neighbor = kd_tree
            .nearest_n_within::<SquaredEuclidean>(&key, threshold, NonZero::new(1).unwrap(), false)
            .first()?;

        Some(cache.get(&nearest_neighbor.item).unwrap().clone())
    }

    fn get_or_shade(&self, frag: F, key: [f64; D]) -> S::Output {
        if self.time_invalidates
            && self.cached_time.load(core::sync::atomic::Ordering::Relaxed) != frag.time().to_bits()
        {
            self.invalidate();
            self.cached_time
                .store(frag.time().to_bits(), core::sync::atomic::Ordering::Relaxed);
        } else if let Some(color) = self.get(key) {
            return color;
        }

        let color = self.shader.shade(frag);

        let item = self
            .item_counter
            .fetch_add(1, core::sync::atomic::Ordering::AcqRel);

        self.cache.write().unwrap().insert(item, color.clone());
        self.kd_tree.write().unwrap().add(&key, item);
        color
    }
}

impl<const D: usize, F: VertexDim<D>, S: Shader<F>> Shader<F> for Memoize<D, F, S>
where
    S::Output: Clone,
{
    type Output = S::Output;

    fn shade(&self, frag: F) -> Self::Output {
        let pos = frag.pos();
        let mut key = [0.0; D];
        key.copy_from_slice(&pos[0..D]);

        self.get_or_shade(frag, key)
    }
}

pub fn memoize<const D: usize, F: VertexDim<D>, S: Shader<F>>(
    shader: S,
    distance_threshold: Option<f64>,
    time_invalidates: bool,
) -> Memoize<D, F, S> {
    Memoize {
        shader,
        threshold: distance_threshold,
        time_invalidates,
        kd_tree: RwLock::new(KdTree::new()),
        cache: RwLock::new(BTreeMap::new()),
        cached_time: AtomicU64::new(0),
        item_counter: AtomicU64::new(0),
    }
}
