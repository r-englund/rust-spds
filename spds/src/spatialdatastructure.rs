pub trait SpatialDataStructure: Default {
    const D: usize;
    type Point;

    fn is_empty(&self) -> bool {
        self.length() == 0
    }

    fn length(&self) -> usize;

    fn add_points<IT>(&mut self, points: IT)
    where
        IT: ExactSizeIterator + IntoIterator<Item = Self::Point>;

    fn find_nearest(&self, p: Self::Point) -> Option<&Self::Point>;
}

#[cfg(test)]
pub(crate) mod tests {
    //use super::*;

    pub fn float_eq_eps<const D: usize>(a: &[f64; D], b: &[f64; D], epsilon: f64) -> bool {
        (0..D)
            .find(|d: &usize| ((a[*d] - b[*d]).abs() - epsilon).is_sign_positive())
            .is_none()
    }

    pub fn float_eq<const D: usize>(a: &[f64; D], b: &[f64; D]) -> bool {
        float_eq_eps::<D>(a, b, std::f64::EPSILON)
    }

    #[macro_export]
    macro_rules! generate_test {
        ($T:ident) => {
            use crate::spatialdatastructure::tests::*;

            #[test]
            fn test_ctor() {
                let _: $T<2> = Default::default();

                assert_eq!(0, 0);
            }

            #[test]
            fn test_add_points() {
                let mut a: $T<1> = Default::default();

                assert!(a.is_empty());
                assert_eq!(0, a.len());

                a.add_points((0..11).map(|x| [(x as f64) / 10.0]));

                assert!(!a.is_empty());
                assert_eq!(11, a.len());
            }

            #[test]
            fn test_find_nearest() {
                let mut a: $T<1> = Default::default();

                assert!(a.find_nearest([0.0]).is_none());

                a.add_points((0..11).map(|x| [(x as f64) / 10.0]));

                let find_nearest_helper = |p, expected| {
                    let res = a.find_nearest([p]);

                    assert!(res.is_some());
                    assert!(float_eq(&[expected], res.unwrap()));
                };

                find_nearest_helper(0.01, 0.0);

                find_nearest_helper(0.2, 0.2);
                find_nearest_helper(0.199, 0.2);
                find_nearest_helper(0.201, 0.2);
                find_nearest_helper(0.2499, 0.2);
                find_nearest_helper(0.150001, 0.2);
            }
        };
    }
}
