#![feature(min_const_generics)]
use rand::seq::SliceRandom;

use spds::vector::*;

use std::{
    fs::File,
    time::{Duration, Instant},
};

use std::fmt::Write as FmtWrite;

use std::io::prelude::*;
//use std::fs::File;

// fn rand_vec<R, const D: usize>(rng: &mut R, n: usize) -> Vec<[f64; D]>
// where
// R: rand::Rng,
// {
//     (0..n).map(|_| [0.0; D].map(|_| rng.gen::<f64>())).collect()
// }

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

fn test_perf<F, const D: usize>(points: &Vec<[f64; D]>, m: usize, f: F) -> Duration
where
    F: Fn(&Vec<[f64; D]>, [f64; D], usize) -> Vec<[f64; D]>,
{
    let mut d: Duration = Default::default();

    (0..20).for_each(|_| {
        let now = Instant::now();
        let _ = f(&points, [0.03; D], m);
        d = d + now.elapsed();
    });
    d / 20
}

struct Result {
    num_points: usize,
    find_amount: usize,
    find_frac: f64,
    duration: Duration,
    method: &'static str,
}

macro_rules! csv_format {
    () => {
        "{1} {0} {2} {0} {3} {0} {4} {0} {5}"
    };
}

fn create_csv<'a, Results>(filename: &str, results: Results)
where
    Results: Iterator<Item = &'a Result>,
{
    let mut s = String::new();

    writeln!(
        &mut s,
        csv_format!(),
        ";", "num_points", "find_amount", "find_frac", "method", "duration"
    )
    .unwrap();

    results.for_each(|res| {
        writeln!(
            &mut s,
            csv_format!(),
            ";",
            res.num_points,
            res.find_amount,
            res.find_frac,
            res.method,
            res.duration.as_nanos()
        )
        .unwrap();
    });

    writeln!(File::create(filename).unwrap(), "{}", s).unwrap();
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut all_results: Vec<Result> = Vec::new();
    let n_sizes: u32 = if cfg!(debug_assertions) { 1 } else { 20 } + 1;

    (1..n_sizes).map(|i| 2usize.pow(i)).for_each(|num_points| {
        let v: Vec<[f64; 1]> = shuffled_inc_vec(&mut rng, num_points);

        print!("{:4}: ", num_points);

        let mut find_fracs = (0..=50)
            .map(|i| {
                let find_frac = i as f64 / 50 as f64;
                let find_amount = ((find_frac * num_points as f64) as usize).max(1);
                (find_frac, find_amount)
            })
            .collect::<Vec<_>>();
        find_fracs.dedup_by_key(|(_, n)| *n);
        find_fracs.drain(..).for_each(|(find_frac, find_amount)| {
            if num_points >= find_amount {
                all_results.push(Result {
                    num_points,
                    find_amount,
                    find_frac,
                    duration: test_perf(&v, find_amount, find_n_nearest),
                    method: "find_n_nearest",
                });
                all_results.push(Result {
                    num_points,
                    find_amount,
                    find_frac,
                    duration: test_perf(&v, find_amount, find_n_nearest_sort),
                    method: "find_n_nearest_sort",
                });
                create_csv("perf1.csv", all_results.iter());
                print!("{:5} ", find_amount);
            }
        });

        //  let subs = (num_points).min(10);

        // (0..=subs)
        //     //         .map(|i| (((i as f64 / subs as f64) * num_points as f64) as usize).max(1))
        //     //.for_each(|find_amount| {
        //     .for_each(|i| {
        //         let find_frac = i as f64 / subs as f64;
        //         let find_amount = ((find_frac * num_points as f64) as usize).max(1);
        //         all_results.push(Result {
        //             num_points,
        //             find_amount,
        //             find_frac,
        //             duration: test_perf(&v, find_amount, find_n_nearest),
        //             method: "find_n_nearest",
        //         });
        //         all_results.push(Result {
        //             num_points,
        //             find_amount,
        //             find_frac,
        //             duration: test_perf(&v, find_amount, find_n_nearest_sort),
        //             method: "find_n_nearest_sort",
        //         });
        //         create_csv("perf1.csv", all_results.iter());
        //         print!("{:5} ", find_amount);
        //     });
        println!("");
    });

    create_csv("perf1.csv", all_results.iter());
}
