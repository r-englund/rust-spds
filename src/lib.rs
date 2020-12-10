#![feature(min_const_generics)]

pub mod orthtree;

pub use crate::orthtree::Orthtree;
pub type Quadtree = Orthtree<2, 8>;
pub type Octree = Orthtree<3, 8>;

#[cfg(test)]
mod tests {

    #[test]
    fn dummy() {
        assert_eq!(1, 1);
    }
}
