//! Tests for all `ArrayList` constructors and the initial state they produce.

extern crate alloc;
use alloc::vec;

use arrlist::ArrayList;

#[test]
fn new_is_empty()
{
	let list: ArrayList<i32> = ArrayList::new();
	assert!(list.is_empty());
}

#[test]
fn new_len_is_zero()
{
	let list: ArrayList<i32> = ArrayList::new();
	assert_eq!(list.len(), 0);
}

#[test]
fn new_capacity_is_zero()
{
	let list: ArrayList<i32> = ArrayList::new();
	assert_eq!(list.capacity(), 0);
}

#[test]
fn with_capacity_sets_capacity()
{
	let list: ArrayList<i32> = ArrayList::with_capacity(8);
	assert_eq!(list.capacity(), 8);
}

#[test]
fn with_capacity_len_is_zero()
{
	let list: ArrayList<i32> = ArrayList::with_capacity(8);
	assert_eq!(list.len(), 0);
}

#[test]
fn with_capacity_zero_behaves_like_new()
{
	let list: ArrayList<i32> = ArrayList::with_capacity(0);
	assert_eq!(list.capacity(), 0);
	assert!(list.is_empty());
}

#[test]
fn from_array_len()
{
	let list = ArrayList::from_array([1, 2, 3]);
	assert_eq!(list.len(), 3);
}

#[test]
fn from_array_elements_in_order()
{
	let list = ArrayList::from_array([10, 20, 30]);
	assert_eq!(list.get(0), Some(&10));
	assert_eq!(list.get(1), Some(&20));
	assert_eq!(list.get(2), Some(&30));
}

#[test]
fn from_array_empty()
{
	let list = ArrayList::from_array([] as [i32; 0]);
	assert!(list.is_empty());
}

#[test]
fn from_vec_len()
{
	let list = ArrayList::from(vec![1, 2, 3]);
	assert_eq!(list.len(), 3);
}

#[test]
fn from_vec_elements_in_order()
{
	let list = ArrayList::from(vec![7, 8, 9]);
	assert_eq!(list.get(0), Some(&7));
	assert_eq!(list.get(2), Some(&9));
}

#[test]
fn from_vec_empty()
{
	let list: ArrayList<i32> = ArrayList::from(vec![]);
	assert!(list.is_empty());
}

#[test]
fn from_slice_len()
{
	let list = ArrayList::from_slice(&[1, 2, 3]);
	assert_eq!(list.len(), 3);
}

#[test]
fn from_slice_elements_match()
{
	let src = [100, 200, 300];
	let list = ArrayList::from_slice(&src);
	assert_eq!(list.get(0), Some(&100));
	assert_eq!(list.get(1), Some(&200));
	assert_eq!(list.get(2), Some(&300));
}

#[allow(unused)]
#[test]
fn from_slice_is_independent_clone()
{
	let mut src = [1, 2, 3];
	let list = ArrayList::from_slice(&src);
	src[0] = 99; // mutating the source should not affect the list
	assert_eq!(list.get(0), Some(&1));
}

#[test]
fn from_array_trait_impl()
{
	let list: ArrayList<i32> = ArrayList::from([1, 2, 3]);
	assert_eq!(list.len(), 3);
}

#[test]
fn from_vec_trait_impl()
{
	let list: ArrayList<i32> = ArrayList::from(vec![4, 5, 6]);
	assert_eq!(list.len(), 3);
}
