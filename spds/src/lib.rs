#![feature(min_const_generics)]
//#![feature(const_generics)]
#![feature(test)]
//#![feature(array_map)]
#![feature(concat_idents)]

pub mod orthtree;
pub mod spatialdatastructure;
pub mod vector;

mod utils;

use crate::orthtree::Orthtree;

pub type Quadtree = Orthtree<2, 8>;
pub type Octree = Orthtree<3, 8>;
