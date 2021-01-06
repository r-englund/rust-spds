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
//impl<'a, F, const D: usize> SpatialDataStructure for &'a Vec<[F; D]>
where
    F: Sub<Output = F> + Sum + Mul<Output = F> + Copy + PartialOrd + Copy + Clone + Sized,
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

    fn find_nearest(&self, p: &Self::Point) -> Option<&Self::Point> {
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

    fn find_n_nearest(&self, p: &Self::Point, n: usize) -> Option<Vec<Self::Point>> {
        Some(find_n_nearest_sort(self, *p, n))
    }

    fn iter_i(&'a self) -> Self::Iter<'a> {
        self.iter()
    }

    fn visit_all_points<Func>(&self, f: Func)
    where
        Func: Fn(&Self::Point),
    {
        self.iter().for_each(f);
    }
}

pub fn find_n_nearest_sort<F, const D: usize>(points: &[[F; D]], p: [F; D], n: usize) -> Vec<[F; D]>
where
    F: Sub<Output = F> + Sum + Mul<Output = F> + Copy + PartialOrd,
{
    find_n_nearest_sort_dist(points, p, n, euclidean_dist_2)
}

pub fn find_n_nearest_sort_dist<F, Distance, const D: usize>(
    points: &[[F; D]],
    p: [F; D],
    n: usize,
    dist: Distance,
) -> Vec<[F; D]>
where
    Distance: Fn(&[F; D], &[F; D]) -> F,
    F: Sub<Output = F> + Sum + Mul<Output = F> + Copy + PartialOrd,
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

pub fn find_n_nearest<F, const D: usize>(points: &[[F; D]], p: [F; D], n: usize) -> Vec<[F; D]>
where
    F: Sub<Output = F> + Sum + Mul<Output = F> + Copy + PartialOrd,
{
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
                    .sum::<F>(),
            )
        })
        .collect::<Vec<_>>();

    let mut res: Vec<[F; D]> = Vec::with_capacity(n);
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
    //use super::*;
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
}
