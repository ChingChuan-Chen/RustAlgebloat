// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use traits::{MatrixRawGet, SameShape};

impl
SameShape for
f64
{
	fn same_shape(&self, _: usize, _: usize) -> bool
	{
		true
	}
}

impl
MatrixRawGet for
f64
{
	unsafe fn raw_get(&self, _: usize, _: usize) -> f64
	{
		*self
	}
}
