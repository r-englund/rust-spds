use std::{
    iter::Sum,
    ops::{Mul, Sub},
};

fn abs_diff<F>(a: F, b: F) -> F
where
    F: Sub<Output = F> + PartialOrd,
{
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn euclidean_dist_2<F, const D: usize>(a: &[F; D], b: &[F; D]) -> F
where
    F: Sub<Output = F> + Sum + Mul<Output = F> + Copy + PartialOrd,
{
    if D == 1 {
        abs_diff(a[0], b[0])
    } else {
        (0..D)
            .map(|dim| {
                let d: F = b[dim] - a[dim];
                d * d
            })
            .sum::<F>()
    }
}

pub fn manhattan_dist<F, const D: usize>(a: &[F; D], b: &[F; D]) -> F
where
    F: Sub<Output = F> + Sum + Copy + PartialOrd,
{
    (0..D).map(|dim| abs_diff(a[dim], b[dim])).sum::<F>()
}

pub fn cmp<F>(a: F, b: F) -> std::cmp::Ordering
where
    F: PartialOrd,
{
    if a < b {
        std::cmp::Ordering::Less
    } else if a > b {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

#[cfg(test)]
pub mod tests {

    use rand::seq::SliceRandom;

    // fn rand_vec<R, const D: usize>(rng: &mut R, n: usize) -> Vec<[f64; D]>
    // where
    // R: rand::Rng,
    // {
    //     (0..n).map(|_| [0.0; D].map(|_| rng.gen::<f64>())).collect()
    // }

    pub fn inc_vec<const D: usize>(n: usize) -> Vec<[f64; D]> {
        let inv = 1.0 / (n - 1) as f64;
        (0..n).map(|i| [i as f64 * inv; D]).collect()
    }

    pub fn shuffled_inc_vec<R, const D: usize>(rng: &mut R, n: usize) -> Vec<[f64; D]>
    where
        R: rand::Rng,
    {
        let mut v = inc_vec::<D>(n);
        v.shuffle(rng);
        v
    }
}
