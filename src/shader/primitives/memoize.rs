use core::sync::atomic::AtomicU64;
use std::{collections::BTreeMap, sync::RwLock};

use crate::shader::{Shader, VertexDim};

type OrderedFloat = ordered_float::OrderedFloat<f64>;

#[derive(Debug)]
pub struct Memoize<const D: usize, F: VertexDim<D>, S: Shader<F>> {
    shader: S,
    cache: RwLock<BTreeMap<[OrderedFloat; D], S::Output>>,
    threshold: Option<f64>,
    time_invalidates: bool,
    cached_time: AtomicU64,
}
impl<const D: usize, F: VertexDim<D>, S: Shader<F>> Memoize<D, F, S>
where
    S::Output: Clone,
{
    fn invalidate(&self) {
        self.cache.write().unwrap().clear();
    }

    fn get(&self, key: [OrderedFloat; D]) -> Option<S::Output> {
        let cache = self.cache.read().unwrap();
        let key = cache.keys().find(|k| {
            k.iter()
                .zip(key.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt()
                < self.threshold.unwrap_or(0.0)
        });
        key.map(|k| (*cache.get(k).unwrap()).clone())
    }

    fn get_or_shade(&self, frag: F, key: [OrderedFloat; D]) -> S::Output {
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
        self.cache.write().unwrap().insert(key, color.clone());
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
        let mut key = [ordered_float::OrderedFloat(0.0); D];
        for i in 0..D {
            key[i] = pos[i].into();
        }

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
        cache: RwLock::new(BTreeMap::new()),
        cached_time: AtomicU64::new(0),
    }
}
