// index_cube.rs (splines library)

use num::{Float};

#[derive(Clone)]
pub struct IndexCube<T,const ND: usize>
{
	vec: Vec<T>,
}

impl<T, const ND: usize> IndexCube<T,ND> {
	
	pub fn new()->Self{
		assert!(ND <= 16);
		return Self{
			vec: Vec::with_capacity((1u16 << ND).into()),
		};
	}
	
	
}