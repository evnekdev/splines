// makima.rs (splines library)

use num::{Float};

use crate::ppdata::{PPData};
use crate::{diff, kernel_conv};

pub fn makima<T: Float + std::fmt::Debug>(xx: &[T], yy: &[T])->PPData<T>{
	//let hh : Vec<T> = diff(xx).collect();
	//let delta : Vec<T> = diff(yy).zip(hh.iter()).map(|(dy,dx)| dy/ *dx).collect();
	//println!("hh = {:?}\ndelta = {:?}\n", &hh, &delta);
	let ss = slopes_makima(xx, yy);
	//println!("xx = {:?}\n, yy = {:?}\n, ss = {:?}\n", xx, yy, &ss);
	return PPData::new(xx, yy, &ss);
}

pub fn slopes_makima<T: Float + std::fmt::Debug>(xx: &[T], yy: &[T])->Vec<T>{
	let hh : Vec<T> = diff(xx).collect();
	let delta : Vec<T> = diff(yy).zip(hh.iter()).map(|(dy,dx)| dy/ *dx).collect();
	// special case of two points, use linear slope
	if xx.len() == 2 {
		return vec![delta[0];2];
	}
	// calculate the missing deltas for points 0, 1, n, n-1.
	let n = xx.len();
	let delta_m1 : T = (delta[0]+delta[0]) - delta[1];
	let delta_m2 : T = (delta_m1+delta_m1) - delta[0];
	let delta_n  : T = (delta[n-2]+delta[n-2]) - delta[n-3];
	let delta_n1 : T = (delta_n+delta_n) - delta[n-2];
	//println!("delta[n-2] = {:?}, delta[n-3] = {:?}", &delta[n-2], &delta[n-3]);
	//println!("delta_n = {:?}, delta_n1 = {:?}", &delta_n, &delta_n1);
	let delta_prefix = vec![delta_m2,delta_m1];
	let delta_suffix = vec![delta_n, delta_n1];
	//println!("delta_prefix = {:?}\ndelta_suffix = {:?}\n", &delta_prefix, &delta_suffix);
	let mut delta_new : Vec<T> = delta_prefix.iter().chain(delta.iter()).chain(delta_suffix.iter()).map(|&v| v).collect();
	//let mut delta_new : Vec<T> = delta.iter().map(|&v| v).collect();
	//println!("delta_new = {:?}", &delta_new);
	let k1 = [-T::one(),T::one()];
	let half : T = T::one()/(T::one() + T::one());
	let k2 = [half, half];
	let it1 : Vec<T> = kernel_conv(&delta_new, &k1).map(|v| v.abs()).collect();
	let it2 : Vec<T> = kernel_conv(&delta_new, &k2).map(|v| v.abs()).collect();
	//println!("it1 ={:?}", &it1);
	//println!("it2 = {:?}", &it2);
	let it1 = kernel_conv(&delta_new, &k1).map(|v| v.abs());
	let it2 = kernel_conv(&delta_new, &k2).map(|v| v.abs());
	let weights : Vec<T> = it1.zip(it2).map(|(v1,v2)| v1+v2).collect();
	//println!("weights = {:?}", &weights);
	let k3 = [T::one(), T::zero(), T::one()];
	let weights12 : Vec<T> = kernel_conv(&weights, &k3).collect();
	//println!("weights12 = {:?}", &weights12);
	let s1 : Vec<T> = weights[2..].iter().zip(delta_new[1..n+1].iter()).map(|(w,d)| *w* *d).collect();
	let s2 : Vec<T> = weights[0..n].iter().zip(delta_new[2..n+2].iter()).map(|(w,d)| *w* *d).collect();
	let ss : Vec<T> = weights12.iter().zip(s1.iter().zip(s2.iter())).map(|(w,(s1,s2))| (*s1+*s2)/ *w).collect();
	//println!("s1 = {:?}, s2 = {:?}, ss = {:?}", &s1, &s2, &ss);
	return ss;
}
/*
pub fn slopes_makima<T: Float + std::fmt::Debug>(xx: &[T], yy: &[T], delta: &[T])->Vec<T>{
	// special case of two points, use linear slope
	if xx.len() == 2 {
		return vec![delta[0];2];
	}
	// calculate the missing deltas for points 0, 1, n, n-1.
	let n = xx.len();
	let delta_m1 : T = (delta[0]+delta[0]) - delta[1];
	let delta_m2 : T = (delta_m1+delta_m1) - delta[0];
	let delta_n  : T = (delta[n-2]+delta[n-2]) - delta[n-3];
	let delta_n1 : T = (delta_n+delta_n) - delta[n-2];
	//println!("delta[n-2] = {:?}, delta[n-3] = {:?}", &delta[n-2], &delta[n-3]);
	//println!("delta_n = {:?}, delta_n1 = {:?}", &delta_n, &delta_n1);
	let delta_prefix = vec![delta_m2,delta_m1];
	let delta_suffix = vec![delta_n, delta_n1];
	//println!("delta_prefix = {:?}\ndelta_suffix = {:?}\n", &delta_prefix, &delta_suffix);
	let mut delta_new : Vec<T> = delta_prefix.iter().chain(delta.iter()).chain(delta_suffix.iter()).map(|&v| v).collect();
	//let mut delta_new : Vec<T> = delta.iter().map(|&v| v).collect();
	//println!("delta_new = {:?}", &delta_new);
	let k1 = [-T::one(),T::one()];
	let half : T = T::one()/(T::one() + T::one());
	let k2 = [half, half];
	let it1 : Vec<T> = kernel_conv(&delta_new, &k1).map(|v| v.abs()).collect();
	let it2 : Vec<T> = kernel_conv(&delta_new, &k2).map(|v| v.abs()).collect();
	//println!("it1 ={:?}", &it1);
	//println!("it2 = {:?}", &it2);
	let it1 = kernel_conv(&delta_new, &k1).map(|v| v.abs());
	let it2 = kernel_conv(&delta_new, &k2).map(|v| v.abs());
	let weights : Vec<T> = it1.zip(it2).map(|(v1,v2)| v1+v2).collect();
	//println!("weights = {:?}", &weights);
	let k3 = [T::one(), T::zero(), T::one()];
	let weights12 : Vec<T> = kernel_conv(&weights, &k3).collect();
	//println!("weights12 = {:?}", &weights12);
	let s1 : Vec<T> = weights[2..].iter().zip(delta_new[1..n+1].iter()).map(|(w,d)| *w* *d).collect();
	let s2 : Vec<T> = weights[0..n].iter().zip(delta_new[2..n+2].iter()).map(|(w,d)| *w* *d).collect();
	let ss : Vec<T> = weights12.iter().zip(s1.iter().zip(s2.iter())).map(|(w,(s1,s2))| (*s1+*s2)/ *w).collect();
	//println!("s1 = {:?}, s2 = {:?}, ss = {:?}", &s1, &s2, &ss);
	return ss;
}
*/