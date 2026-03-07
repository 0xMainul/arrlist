//! Tests for `IntoIter`, `Iter`, and `IterMut`.

extern crate alloc;
use alloc::vec::Vec;

use arrlist::ArrayList;

#[test]
fn into_iter_yields_all_elements_in_order()
{
	let list = ArrayList::from_array([1, 2, 3]);
	let collected: Vec<i32> = list.into_iter().collect();
	assert_eq!(collected, [1, 2, 3]);
}

#[test]
fn into_iter_empty_list()
{
	let list: ArrayList<i32> = ArrayList::new();
	let collected: Vec<i32> = list.into_iter().collect();
	assert!(collected.is_empty());
}

#[test]
fn into_iter_for_loop()
{
	let list = ArrayList::from_array([10, 20, 30]);
	let mut sum = 0;
	for val in list
	{
		sum += val;
	}
	assert_eq!(sum, 60);
}

#[test]
fn into_iter_single_element()
{
	let list = ArrayList::from_array([42]);
	let mut iter = list.into_iter();
	assert_eq!(iter.next(), Some(42));
	assert_eq!(iter.next(), None);
}

#[test]
fn into_iter_next_returns_none_when_exhausted()
{
	let list = ArrayList::from_array([1]);
	let mut iter = list.into_iter();
	iter.next();
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), None); // repeated calls must stay None
}

#[test]
fn iter_yields_shared_refs_in_order()
{
	let list = ArrayList::from_array([1, 2, 3]);
	let collected: Vec<&i32> = list.iter().collect();
	assert_eq!(collected, [&1, &2, &3]);
}

#[test]
fn iter_does_not_consume_list()
{
	let list = ArrayList::from_array([1, 2, 3]);
	let _ = list.iter().count();
	// list is still usable after iteration
	assert_eq!(list.len(), 3);
}

#[test]
fn iter_empty_list()
{
	let list: ArrayList<i32> = ArrayList::new();
	assert_eq!(list.iter().next(), None);
}

#[test]
fn iter_ref_for_loop()
{
	let list = ArrayList::from_array([1, 2, 3]);
	let mut sum = 0;
	for val in &list
	{
		sum += val;
	}
	assert_eq!(sum, 6);
}

#[test]
fn iter_can_be_called_multiple_times()
{
	let list = ArrayList::from_array([1, 2, 3]);
	assert_eq!(list.iter().count(), 3);
	assert_eq!(list.iter().count(), 3);
}

#[test]
fn iter_next_returns_none_when_exhausted()
{
	let list = ArrayList::from_array([1]);
	let mut iter = list.iter();
	iter.next();
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut_yields_mutable_refs_in_order()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	let collected: Vec<i32> = list.iter_mut().map(|x| *x).collect();
	assert_eq!(collected, [1, 2, 3]);
}

#[test]
fn iter_mut_allows_mutation()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	for val in list.iter_mut()
	{
		*val *= 10;
	}
	assert_eq!(list.get(0), Some(&10));
	assert_eq!(list.get(1), Some(&20));
	assert_eq!(list.get(2), Some(&30));
}

#[test]
fn iter_mut_empty_list()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	assert_eq!(list.iter_mut().next(), None);
}

#[test]
fn iter_mut_ref_for_loop()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	for val in &mut list
	{
		*val += 100;
	}
	assert_eq!(list.get(0), Some(&101));
	assert_eq!(list.get(2), Some(&103));
}

#[test]
fn iter_mut_next_returns_none_when_exhausted()
{
	let mut list = ArrayList::from_array([1]);
	let mut iter = list.iter_mut();
	iter.next();
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), None);
}
