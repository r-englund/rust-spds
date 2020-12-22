#![allow(dead_code)]

// use crate::utils;

// use Self::Orthant;
// use Self::Point;
// use Self::Tree;

trait TreeImpl {
    type Tree;
    type Orthant;
    type Point;
}

struct Orthant<const D: usize, const N: usize> {
    lower_bounds: [f64; D],
    upper_bounds: [f64; D],

    points: Vec<usize>,

    children: Vec<usize>,
    // children: Option<&[usize; 2^D]>
}

pub struct Orthtree<const D: usize, const N: usize> {
    points: Vec<[f64; D]>,

    orthants: Vec<Orthant<D, N>>,
}

impl<const D: usize, const N: usize> Orthant<D, N> {
    fn new(lower_bounds: [f64; D], upper_bounds: [f64; D]) -> Self {
        Self {
            lower_bounds,
            upper_bounds,
            points: Vec::with_capacity(N),
            children: Vec::with_capacity(0x1 << D),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn add_points<IT>(&mut self, tree: &mut Orthtree<D, N>, points: IT)
    where
        IT: ExactSizeIterator + IntoIterator<Item = [f64; D]>,
    {
        if self.is_leaf() && points.len() + self.points.len() <= N {
            let i0 = tree.points.len();
            tree.points.extend(points);
            let i1 = tree.points.len();
            self.points.extend(i0..i1);
        } else {
            // let i = tree.orthants.len();
            // tree.orthants.append(&mut self.subdived());
            // tree.orthants.get_mut(i..).iter_mut().for
        }
    }

    fn subdived(&self) -> Vec<Self> {
        (0..0x1 << D)
            .map(|subdiv| {
                let mut l_bound = [0.0; D];
                let mut u_bound = [0.0; D];

                (0..D).for_each(|dim| {
                    let m = 0.5 * (self.lower_bounds[dim] + self.upper_bounds[dim]);
                    l_bound[dim] = if subdiv & (0b1 << dim) != 0 {
                        self.lower_bounds[dim]
                    } else {
                        m
                    };
                    u_bound[dim] = if subdiv & (0b1 << dim) != 0 {
                        m
                    } else {
                        self.upper_bounds[dim]
                    };
                });

                Self::new(l_bound, u_bound)
            })
            .collect()
    }

    fn is_inside(&self, p: &[f64; D]) -> bool {
        p.lt(&self.upper_bounds) && p.ge(&self.lower_bounds)
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

impl<const D: usize, const N: usize> Orthtree<D, N> {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            orthants: vec![Orthant::new([0.0; D], [0.0; D])],
        }
    }

    fn get_bounds(&self) -> ([f64; D], [f64; D]) {
        self.orthants.first().unwrap().get_bounds()
    }
    fn get_lower_bounds(&self) -> [f64; D] {
        self.orthants.first().unwrap().get_lower_bounds()
    }
    fn get_upper_bounds(&self) -> [f64; D] {
        self.orthants.first().unwrap().get_upper_bounds()
    }
    fn get_bound<const AXIS: usize>(&self) -> (f64, f64) {
        self.orthants.first().unwrap().get_bound::<AXIS>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
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

        #[test]
        fn test_name2() {
            let _a: Orthant<2, 8> = Orthant::new(
                [0.0; 2], //
                [1.0; 2],
            );

            let _tree: Orthtree<2, 8> = Orthtree::new();
            //_a.add_points(&mut _tree, vec![[0.0, 0.0], [1.0, 1.0], [0.5, 0.5]].iter());
        }
    }

    #[cfg(test)]
    mod orthtree {
        use super::*;

        #[test]
        fn asdf() {
            let _tree: Orthtree<2, 8> = Orthtree::new();
        }
    }
}
