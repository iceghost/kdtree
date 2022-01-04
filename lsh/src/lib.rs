use kd_tree::{DissimilarityQueue, Neighbor};
use multi_dimension::distances::{dissimilarity_between, DissimilarityMeasure};
use point_3d::{Float, Point};
use rand::Rng;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

const K: usize = 2;
const L: usize = 10;
const W: f32 = 2.0;

pub struct EuclidianLSHSearcher<'a> {
    l_hashers: Vec<Box<dyn Fn(&Point) -> u64>>,
    hash_tables: Vec<HashMap<u64, Vec<&'a Point>>>,
}

impl<'a> std::fmt::Debug for EuclidianLSHSearcher<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.hash_tables.fmt(f)
    }
}

impl<'a> FromIterator<&'a Point> for EuclidianLSHSearcher<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Point>>(iter: T) -> Self {
        let points = iter.into_iter().collect::<Vec<_>>();
        let l_hashers = (0..L).map(|_| k_euclidian_hash()).collect::<Vec<_>>();

        let mut hash_tables = Vec::with_capacity(L);
        for l in 0..L {
            let k_hash = &l_hashers[l];
            let mut hash_table: HashMap<u64, Vec<&Point>> = HashMap::new();
            for point in points.iter() {
                hash_table
                    .entry(k_hash(point))
                    .and_modify(|e| {
                        e.push(&point);
                    })
                    .or_insert(vec![&point]);
            }
            hash_tables.push(hash_table);
        }

        Self {
            l_hashers,
            hash_tables,
        }
    }
}

impl<'a> EuclidianLSHSearcher<'a> {
    pub fn search(&'a self, searchee: &'a Point) -> impl Iterator<Item = Neighbor<&'a Point, Float>> {
        let mut queue = DissimilarityQueue::<Neighbor<&Point, Float>>::with_capacity(1);
        for (hasher, table) in itertools::zip(self.l_hashers.iter(), self.hash_tables.iter()) {
            let key = hasher(searchee);
            let potentials = table.get(&key);
            let potentials = if let Some(potentials) = potentials {
                potentials
            } else {
                continue;
            };
            for potential in potentials {
                queue.push(Neighbor(
                    potential,
                    dissimilarity_between(searchee, potential),
                ));
            }
        }
        queue.into_iter()
    }
}

fn k_euclidian_hash() -> Box<dyn Fn(&Point) -> u64> {
    let funcs = (0..K).map(|_| euclidian_hash()).collect::<Vec<_>>();
    Box::new(move |point| {
        let mut hasher = DefaultHasher::new();
        for func in funcs.iter() {
            func(point).hash(&mut hasher);
        }
        hasher.finish()
    })
}

fn euclidian_hash() -> impl Fn(&Point) -> isize {
    let mut rng = rand::thread_rng();
    let x = rng.gen::<f32>() - 0.5;
    let y = rng.gen::<f32>() - 0.5;
    let z = rng.gen::<f32>() - 0.5;
    let b = rng.gen_range(0.0..=W);
    let dir = Point::new(x, y, z);
    move |point| hash_point(&dir, Float::new(b), Float::new(W), point)
}

fn hash_point(dir: &Point, b: Float, w: Float, point: &Point) -> isize {
    // project into line: (unit * point + b) / w
    ((Point::dot(&dir, &point) / dir.magnitude() + b) / w).floor() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn random_point() -> Point {
            Point::new(
                rand::random::<f32>() * 100f32,
                rand::random::<f32>() * 100f32,
                rand::random::<f32>() * 100f32,
            )
        }

        let points = (0..10000).map(|_| random_point()).collect::<Vec<_>>();

        let searcher = points.iter().collect::<EuclidianLSHSearcher>();

        let seachee = Point::new(50.0, 50.0, 50.0);

        let mut result = searcher.search(&seachee);

        println!("{:?}", result.next());
    }
}
