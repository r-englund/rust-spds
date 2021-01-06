pub trait SpatialDataStructure {
    const D: usize;
    type Point;
    type Iter;

    fn is_empty(&self) -> bool {
        self.length() == 0
    }

    fn length(&self) -> usize;

    fn add_points<IT>(&mut self, points: IT)
    where
        IT: ExactSizeIterator + IntoIterator<Item = Self::Point>;

    fn find_nearest(&self, p: &Self::Point) -> Option<&Self::Point>;

    fn find_n_nearest(&self, p: &Self::Point, n: usize) -> Option<Vec<Self::Point>>;

    fn iter_i(&self) -> Self::Iter;

    fn visit_all_points<F>(&self, f: F)
    where
        F: Fn(&Self::Point);
}

#[cfg(test)]
pub(crate) mod tests {
    #[macro_export]
    macro_rules! generate_type_dim_test {
        ($S:ident,$F:ident,$D:expr) => {
            mod $F {
                use super::*;

                type P = [$F; $D];

                #[test]
                fn test_ctor() {
                    let _: $S<P> = Default::default();

                    assert_eq!(0, 0);
                }

                #[test]
                fn test_add_points() {
                    let mut a: $S<P> = Default::default();

                    assert!(a.is_empty());
                    assert_eq!(0, a.len());

                    //a.add_points((0..11).map(|x| [(x as $F) / 10.0]));

                    assert!(!a.is_empty());
                    assert_eq!(11, a.len());
                }

                #[test]
                fn test_find_nearest() {
                    let mut a: $S<P> = Default::default();

                    //assert!(a.find_nearest([0.0]).is_none());

                    // a.add_points((0..11).map(|x| [(x as $F) / 10.0]));

                    let find_nearest_helper = |p, expected| {
                        //  let res = a.find_nearest([p]);

                        // assert!(res.is_some());
                        // assert!(crate::utils::float_eq(&[expected], res.unwrap()));
                    };

                    find_nearest_helper(0.01, 0.0);

                    find_nearest_helper(0.2, 0.2);
                    find_nearest_helper(0.199, 0.2);
                    find_nearest_helper(0.201, 0.2);
                    find_nearest_helper(0.2499, 0.2);
                    find_nearest_helper(0.150001, 0.2);
                }

                #[test]
                fn test_find_n_nearest() {
                    let _points = gen_test_points(Default::default(), 11);

                    //a.add_points((0..11).map(|x| [(x as $F) / 10.0]));
                }

                struct MultiDimIterator<const D: usize> {
                    cur: [usize; D],
                    limits: [usize; D],
                }

                impl<const D: usize> MultiDimIterator<D> {
                    pub fn new(limits: [usize; D]) -> Self {
                        Self {
                            cur: [0; D],
                            limits,
                        }
                    }
                }

                impl<const D: usize> Iterator for MultiDimIterator<D> {
                    type Item = [usize; D];

                    fn next(&mut self) -> Option<Self::Item> {
                        None
                    }
                }

                impl<const D: usize> ExactSizeIterator for MultiDimIterator<D> {
                    fn len(&self) -> usize {
                        self.limits.iter().fold(1, |a, b| a * b)
                    }
                }

                fn gen_test_points(mut cont: $S<P>, n: u16) -> $S<P> {
                    cont.add_points(MultiDimIterator::new([n as usize; $D]).map(|_| {
                        let mut res: P = Default::default();
                        //res.iter_mut().for_each(|x: &mut $F| {});

                        res
                    }));

                    // MultiDimIterator::new([n as usize; $D]).iter();

                    for cur in MultiDimIterator::new([n as usize; $D]) {
                        let pos: P = Default::default();
                    }

                    // cont.add_points((0..n).map(|i: u16| {
                    //     let x: $F = i.into();
                    //     let mut p: P = Default::default();

                    //     (0..$D).for_each(|y: u16| {
                    //         let x: $F = i.into();
                    //     });
                    //     p
                    // }));
                    cont
                }
            }
        };
    }

    #[macro_export]
    macro_rules! generate_test {
        ($T:ident) => {
            mod dim1 {
                use super::*;
                crate::generate_type_dim_test!($T, f32, 1);
                crate::generate_type_dim_test!($T, f64, 1);
            }
            mod dim2 {
                use super::*;
                crate::generate_type_dim_test!($T, f64, 2);
            }
        };
    }
}
