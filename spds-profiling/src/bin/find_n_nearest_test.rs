#![feature(min_const_generics)]
use std::{env, process::exit};

use rand::seq::SliceRandom;

use std::time::Instant;

fn inc_vec<const D: usize>(n: usize) -> Vec<[f64; D]> {
    let inv = 1.0 / (n - 1) as f64;
    (0..n).map(|i| [i as f64 * inv; D]).collect()
}

fn shuffled_inc_vec<R, const D: usize>(rng: &mut R, n: usize) -> Vec<[f64; D]>
where
    R: rand::Rng,
{
    let mut v = inc_vec::<D>(n);
    v.shuffle(rng);
    v
}

fn main() {
    if env::args().len() != 4 {
        println!(
            "Usage: {} num_points num_find method_index",
            env::args().next().unwrap_or_default()
        );
        exit(1);
    } else {
        let mut rng = rand::thread_rng();

        let num_points = env::args().nth(1).unwrap().parse::<usize>().unwrap();
        let num_find = env::args().nth(2).unwrap().parse::<usize>().unwrap();
        let method_index = env::args().nth(3).unwrap().parse::<u8>().unwrap();

        let points: Vec<[f64; 1]> = shuffled_inc_vec(&mut rng, num_points);

        let finder = match method_index {
            0 => spds::vector::find_n_nearest,
            _ => spds::vector::find_n_nearest_sort,
        };

        let now = Instant::now();
        let _ = finder(&points, [0.5], num_find);
        println!("{}", now.elapsed().as_secs_f64());
    }
}
