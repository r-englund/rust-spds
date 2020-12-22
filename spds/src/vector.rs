//extern crate test;

use std::{
    iter::Sum,
    ops::{Mul, Sub},
};

use crate::{
    spatialdatastructure::SpatialDataStructure,
    utils::{cmp, euclidean_dist_2},
};

impl<F, const D: usize> SpatialDataStructure for Vec<[F; D]>
where
    F: Sub<Output = F> + Sum + Mul<Output = F> + Copy + PartialOrd,
{
    const D: usize = D;
    type Point = [F; D];
    fn length(&self) -> usize {
        self.len()
    }

    fn add_points<IT>(&mut self, points: IT)
    where
        IT: ExactSizeIterator + IntoIterator<Item = [F; D]>,
    {
        self.extend(points);
    }

    fn find_nearest(&self, p: Self::Point) -> Option<&Self::Point> {
        if self.is_empty() {
            return None;
        }

        let index = self
            .iter()
            .map(|c| euclidean_dist_2(&p, c))
            .enumerate()
            .min_by(|(_, a), (_, b)| cmp(*a, *b))
            .unwrap()
            .0;
        self.get(index)
    }
}

pub fn find_n_nearest_sort<const D: usize>(
    points: &[[f64; D]],
    p: [f64; D],
    n: usize,
) -> Vec<[f64; D]> {
    find_n_nearest_sort_dist(points, p, n, euclidean_dist_2)
}

pub fn find_n_nearest_sort_dist<Distance, const D: usize>(
    points: &[[f64; D]],
    p: [f64; D],
    n: usize,
    dist: Distance,
) -> Vec<[f64; D]>
where
    Distance: Fn(&[f64; D], &[f64; D]) -> f64,
{
    let mut ordering = points
        .iter()
        .map(|a| dist(&p, a))
        .enumerate()
        .collect::<Vec<_>>();

    ordering.sort_unstable_by(|(_, a), (_, b)| cmp(*a, *b));

    ordering
        .get(0..n)
        .unwrap()
        .iter()
        .map(|(i, _)| *points.get(*i).unwrap())
        .collect()
}

pub fn find_n_nearest<const D: usize>(points: &[[f64; D]], p: [f64; D], n: usize) -> Vec<[f64; D]> {
    let mut distances = points
        .iter()
        .enumerate()
        .map(|(i, a)| {
            (
                i,
                (0..D)
                    .map(|dim| {
                        let d = p[dim] - a[dim];
                        d * d
                    })
                    .sum::<f64>(),
            )
        })
        .collect::<Vec<_>>();

    let mut res: Vec<[f64; D]> = Vec::with_capacity(n);
    while res.len() < n {
        let (di, (i, _)) = distances
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| cmp(a.1, b.1))
            .unwrap();
        res.push(points[*i]);
        let last = distances.len() - 1;
        distances.swap(di, last);
        distances.truncate(last);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;

    // fn bench_find_n_nearest_sort<const D: usize>(b: &mut Bencher, n: usize) {
    //     let mut rng = rand::thread_rng();

    //     let _v: Vec<[f64; D]> = crate::utils::tests::shuffled_inc_vec(&mut rng, n as usize);

    //     b.iter(|| {
    //         find_n_nearest_sort(&_v, [0.03; D], 9);
    //     });
    // }

    // fn bench_find_n_nearest<const D: usize>(b: &mut Bencher, n: usize) {
    //     let mut rng = rand::thread_rng();

    //     let _v: Vec<[f64; D]> = crate::utils::tests::shuffled_inc_vec(&mut rng, n as usize);

    //     b.iter(|| {
    //         find_n_nearest(&_v, [0.03; D], 9);
    //     });
    // }

    // #[bench]
    // fn bench_find_n_nearest_10_sort(b: &mut Bencher) {
    //     bench_find_n_nearest_sort::<1>(b, 10);
    // }

    // #[bench]
    // fn bench_find_n_nearest_100_sort(b: &mut Bencher) {
    //     bench_find_n_nearest_sort::<1>(b, 100);
    // }

    // #[bench]
    // fn bench_find_n_nearest_1000_sort(b: &mut Bencher) {
    //     bench_find_n_nearest_sort::<1>(b, 1000);
    // }

    // #[bench]
    // fn bench_find_n_nearest_10(b: &mut Bencher) {
    //     bench_find_n_nearest::<1>(b, 10);
    // }

    // #[bench]
    // fn bench_find_n_nearest_100(b: &mut Bencher) {
    //     bench_find_n_nearest::<1>(b, 100);
    // }

    // #[bench]
    // fn bench_find_n_nearest_1000(b: &mut Bencher) {
    //     bench_find_n_nearest::<1>(b, 1000);
    // }

    type Asdf<const D: usize> = Vec<[f64; D]>;
    crate::generate_test!(Asdf);

    // type Asdf = Vec<[f64; 2]>;
    // crate::generate_test!(Asdf);
}
