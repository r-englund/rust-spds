#![allow(dead_code)]

struct Orthant<const D: usize, const N: usize> {
    lower_bounds: [f64; D],
    upper_bounds: [f64; D],
}

impl<const D: usize, const N: usize> Orthant<D, N> {
    fn new(lower_bounds: [f64; D], upper_bounds: [f64; D]) -> Self {
        Self {
            lower_bounds,
            upper_bounds,
        }
    }

    fn get_bounds(&self) -> ([f64; D], [f64; D]) {
        (self.lower_bounds, self.upper_bounds)
    }

    fn get_lower_bounds(&self) -> [f64; D] {
        self.lower_bounds
    }

    fn get_upper_bounds(&self) -> [f64; D] {
        self.upper_bounds
    }

    fn get_bound<const AXIS: usize>(&self) -> (f64, f64) {
        (self.lower_bounds[AXIS], self.upper_bounds[AXIS])
    }
}

pub struct Orthtree<const D: usize, const N: usize> {
    data: Vec<[f64; D]>,

    orthant: Orthant<D, N>,
}

impl<const D: usize, const N: usize> Orthtree<D, N> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            orthant: Orthant::new([0.0; D], [0.0; D]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod orthant {
        use super::*;

        #[test]
        fn test_name() {
            let a: Orthant<2, 8> = Orthant::new(
                [0.0; 2], //
                [1.0; 2],
            );

            assert_eq!(([0.0; 2], [1.0; 2]), a.get_bounds());

            assert_eq!(a.get_bounds(), (a.get_lower_bounds(), a.get_upper_bounds()));

            assert_eq!(0.0, a.get_lower_bounds()[0]);
            assert_eq!(0.0, a.get_lower_bounds()[1]);
            assert_eq!(1.0, a.get_upper_bounds()[0]);
            assert_eq!(1.0, a.get_upper_bounds()[1]);

            assert_eq!((0.0, 1.0), a.get_bound::<0>());
            assert_eq!((0.0, 1.0), a.get_bound::<1>());
        }
    }
}
