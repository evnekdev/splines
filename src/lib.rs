// lib.rs (splines library)

pub mod solve;
pub mod pchip;
pub mod makima;
pub mod ppdata;
pub mod binsearch;
pub mod index_cube;
pub mod search_tree;

use num::{Float};

pub use crate::makima::{makima};
pub use crate::pchip::{pchip};
pub use crate::binsearch::{binary_search_interval};
pub use crate::ppdata::{PPData, MPPData, load_mpp_from_csv};
pub use crate::search_tree::{SearchNode,SearchTree};

pub fn diff<T: Float>(slc: &[T])->impl Iterator<Item=T> + '_{
	return slc.windows(2).map(|w| w[1]-w[0]);
}

pub fn kernel_conv<'a, T: Float>(slc: &'a [T], kernel: &'a [T])->impl Iterator<Item=T> +'a {
	return slc.windows(kernel.len()).map(|w| _kernel_mult(w, kernel));
}


fn _kernel_mult<T: Float>(window: &[T], kernel: &[T])->T{
	return window.iter().zip(kernel.iter()).map(|(w,k)| *w**k).fold(T::zero(), |acc, num| acc + num);
}
/// check if a value is inside the interval
fn _interval_inside<T: Float>(val: &T, vals: (&T,&T))->bool{
	if val == vals.0 || val == vals.1 {return true;}
	let b1 = val > vals.0;
	let b2 = val > vals.1;
	return (vals.1 > vals.0 && b1 && !b2) || (vals.0 > vals.1 && b2 && !b1);
}

pub fn slice_locator<T: Float>(slc: &[T], loc: usize)->T{
	return slc[loc];
}

/********************************************************************************************************************/

