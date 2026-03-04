//! Tests for the `arrlist![]` macro.

extern crate alloc;

use arrlist::{
	ArrayList,
	arrlist,
};

#[test]
fn macro_empty_is_empty()
{
	let list: ArrayList<i32> = arrlist![];
	assert!(list.is_empty());
}

#[test]
fn macro_empty_len_is_zero()
{
	let list: ArrayList<i32> = arrlist![];
	assert_eq!(list.len(), 0);
}

#[test]
fn macro_empty_capacity_is_zero()
{
	let list: ArrayList<i32> = arrlist![];
	assert_eq!(list.capacity(), 0);
}

#[test]
fn macro_list_len()
{
	let list = arrlist![1, 2, 3];
	assert_eq!(list.len(), 3);
}

#[test]
fn macro_list_elements_in_order()
{
	let list = arrlist![10, 20, 30];
	assert_eq!(list.get(0), Some(&10));
	assert_eq!(list.get(1), Some(&20));
	assert_eq!(list.get(2), Some(&30));
}

#[test]
fn macro_list_exact_capacity()
{
	// The macro should pre-allocate exactly as many slots as there are elements.
	let list = arrlist![1, 2, 3, 4, 5];
	assert_eq!(list.capacity(), 5);
}

#[test]
fn macro_list_single_element()
{
	let list = arrlist![42];
	assert_eq!(list.len(), 1);
	assert_eq!(list.get(0), Some(&42));
}

#[test]
fn macro_list_trailing_comma_is_accepted()
{
	let list = arrlist![1, 2, 3,];
	assert_eq!(list.len(), 3);
}

#[test]
fn macro_list_strings()
{
	let list = arrlist!["hello", "world"];
	assert_eq!(list.get(0), Some(&"hello"));
	assert_eq!(list.get(1), Some(&"world"));
}

#[test]
fn macro_repeat_len()
{
	let list = arrlist![0_i32; 5];
	assert_eq!(list.len(), 5);
}

#[test]
fn macro_repeat_all_elements_equal()
{
	let list = arrlist![7_i32; 4];
	for i in 0..4
	{
		assert_eq!(list.get(i), Some(&7));
	}
}

#[test]
fn macro_repeat_capacity_equals_n()
{
	let list = arrlist![0_i32; 8];
	assert_eq!(list.capacity(), 8);
}

#[test]
fn macro_repeat_n_zero_is_empty()
{
	let list = arrlist![99_i32; 0];
	assert!(list.is_empty());
}

#[test]
fn macro_repeat_n_one()
{
	let list = arrlist![42_i32; 1];
	assert_eq!(list.len(), 1);
	assert_eq!(list.get(0), Some(&42));
}

#[test]
fn macro_repeat_clones_value()
{
	// Uses a String to confirm Clone is called correctly.
	extern crate alloc;
	use alloc::string::String;

	let list = arrlist![String::from("hi"); 3];
	assert_eq!(list.len(), 3);
	assert_eq!(list.get(0).map(|s| s.as_str()), Some("hi"));
	assert_eq!(list.get(2).map(|s| s.as_str()), Some("hi"));
}
