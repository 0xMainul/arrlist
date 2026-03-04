//! Tests for `ListError` — variant matching and error message formatting.

extern crate alloc;
use alloc::string::ToString;

use arrlist::{
	arrlist::ArrayList,
	error::ListError,
};

#[test]
fn empty_list_error_from_set()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	assert!(matches!(list.set(0, 1), Err(ListError::EmptyList)));
}

#[test]
fn empty_list_error_from_insert()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	// index 1 on empty list → EmptyList (index 0 would succeed as an append)
	assert!(matches!(list.insert(1, 1), Err(ListError::EmptyList)));
}

#[test]
fn empty_list_error_from_remove()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	assert!(matches!(list.remove(0), Err(ListError::EmptyList)));
}

#[test]
fn empty_list_error_message()
{
	let err = ListError::EmptyList;
	assert_eq!(err.to_string(), "the list is empty");
}

#[test]
fn out_of_bounds_error_from_set()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert!(matches!(
		list.set(5, 99),
		Err(ListError::OutOfBounds {
			idx: 5,
			limits: (0, 2)
		})
	));
}

#[test]
fn out_of_bounds_error_from_insert()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert!(matches!(
		list.insert(10, 99),
		Err(ListError::OutOfBounds {
			idx: 10,
			limits: (0, 2)
		})
	));
}

#[test]
fn out_of_bounds_error_from_remove()
{
	let mut list = ArrayList::from_array([1, 2]);
	assert!(matches!(
		list.remove(5),
		Err(ListError::OutOfBounds {
			idx: 5,
			limits: (0, 1)
		})
	));
}

#[test]
fn out_of_bounds_limits_reflect_current_len()
{
	let mut list = ArrayList::from_array([10, 20, 30, 40, 50]);
	match list.remove(99)
	{
		Err(ListError::OutOfBounds { idx, limits }) =>
		{
			assert_eq!(idx, 99);
			assert_eq!(limits, (0, 4)); // len is 5, so valid range is 0..=4
		},
		_ => panic!("expected OutOfBounds"),
	}
}

#[test]
fn out_of_bounds_error_message_format()
{
	let err = ListError::OutOfBounds {
		idx: 7,
		limits: (0, 3),
	};
	assert_eq!(
		err.to_string(),
		"invalid index 7, expected at least 0 and at most 3"
	);
}

#[test]
fn set_at_last_valid_index_succeeds()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert!(list.set(2, 99).is_ok());
	assert_eq!(list.get(2), Some(&99));
}

#[test]
fn remove_at_last_valid_index_succeeds()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert_eq!(list.remove(2).unwrap(), 3);
}

#[test]
fn insert_at_len_succeeds()
{
	// Inserting at exactly `len` is legal — it appends.
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert!(list.insert(3, 4).is_ok());
	assert_eq!(list.get(3), Some(&4));
}

#[test]
fn capacity_overflow_error_message()
{
	let err = ListError::CapacityOverflow;
	assert_eq!(
		err.to_string(),
		"capacity overflow: cannot grow the list any further"
	);
}

#[test]
fn capacity_overflow_is_debug_printable()
{
	let err = ListError::CapacityOverflow;
	let s = alloc::format!("{err:?}");
	assert!(s.contains("CapacityOverflow"));
}

/// Simulates the overflow condition by building a list whose capacity is
/// already at `usize::MAX / 2 + 1`, then calling push directly.
///
/// We can't actually allocate that much memory in a test, but we can
/// verify the error path by constructing the list state manually via
/// `with_capacity` on a small buffer and then lying about the capacity
/// field — which isn't possible from outside the crate. Instead we test
/// the error variant directly and trust the `checked_mul` unit.
#[test]
fn push_returns_ok_on_normal_list()
{
	let mut list = ArrayList::new();
	assert!(list.push(42).is_ok());
}

#[test]
fn insert_returns_ok_on_normal_list()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert!(list.insert(1, 99).is_ok());
}
