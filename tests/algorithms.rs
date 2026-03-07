//! Tests for the utility algorithms: `sort`, `reverse`, `linear_search`, and
//! `binary_search`.

use arrlist::ArrayList;

#[test]
fn sort_ascending()
{
	let mut list = ArrayList::from_array([5, 3, 1, 4, 2]);
	list.sort();
	assert_eq!(list.get(0), Some(&1));
	assert_eq!(list.get(1), Some(&2));
	assert_eq!(list.get(2), Some(&3));
	assert_eq!(list.get(3), Some(&4));
	assert_eq!(list.get(4), Some(&5));
}

#[test]
fn sort_already_sorted_is_a_noop()
{
	let mut list = ArrayList::from_array([1, 2, 3, 4, 5]);
	list.sort();
	assert_eq!(list.get(0), Some(&1));
	assert_eq!(list.get(4), Some(&5));
}

#[test]
fn sort_reverse_sorted()
{
	let mut list = ArrayList::from_array([5, 4, 3, 2, 1]);
	list.sort();
	assert_eq!(list.get(0), Some(&1));
	assert_eq!(list.get(4), Some(&5));
}

#[test]
fn sort_single_element_is_a_noop()
{
	let mut list = ArrayList::from_array([42]);
	list.sort();
	assert_eq!(list.get(0), Some(&42));
}

#[test]
fn sort_empty_list_does_not_panic()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	list.sort(); // must not panic
}

#[test]
fn sort_with_duplicates()
{
	let mut list = ArrayList::from_array([3, 1, 2, 1, 3]);
	list.sort();
	assert_eq!(list.get(0), Some(&1));
	assert_eq!(list.get(1), Some(&1));
	assert_eq!(list.get(2), Some(&2));
	assert_eq!(list.get(3), Some(&3));
	assert_eq!(list.get(4), Some(&3));
}

#[test]
fn sort_preserves_len()
{
	let mut list = ArrayList::from_array([4, 2, 3, 1]);
	list.sort();
	assert_eq!(list.len(), 4);
}

#[test]
fn reverse_even_length()
{
	let mut list = ArrayList::from_array([1, 2, 3, 4]);
	list.reverse();
	assert_eq!(list.get(0), Some(&4));
	assert_eq!(list.get(1), Some(&3));
	assert_eq!(list.get(2), Some(&2));
	assert_eq!(list.get(3), Some(&1));
}

#[test]
fn reverse_odd_length()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	list.reverse();
	assert_eq!(list.get(0), Some(&3));
	assert_eq!(list.get(1), Some(&2));
	assert_eq!(list.get(2), Some(&1));
}

#[test]
fn reverse_single_element_is_a_noop()
{
	let mut list = ArrayList::from_array([99]);
	list.reverse();
	assert_eq!(list.get(0), Some(&99));
}

#[test]
fn reverse_empty_list_does_not_panic()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	list.reverse(); // must not panic
}

#[test]
fn reverse_twice_is_identity()
{
	let mut list = ArrayList::from_array([1, 2, 3, 4, 5]);
	list.reverse();
	list.reverse();
	assert_eq!(list.get(0), Some(&1));
	assert_eq!(list.get(4), Some(&5));
}

#[test]
fn reverse_preserves_len()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	list.reverse();
	assert_eq!(list.len(), 3);
}

#[test]
fn linear_search_finds_element()
{
	let list = ArrayList::from_array([10, 20, 30]);
	assert_eq!(list.linear_search(&20), Some(1));
}

#[test]
fn linear_search_returns_first_occurrence()
{
	let list = ArrayList::from_array([5, 3, 5, 7]);
	// Should return the first index, not the second.
	assert_eq!(list.linear_search(&5), Some(0));
}

#[test]
fn linear_search_first_element()
{
	let list = ArrayList::from_array([1, 2, 3]);
	assert_eq!(list.linear_search(&1), Some(0));
}

#[test]
fn linear_search_last_element()
{
	let list = ArrayList::from_array([1, 2, 3]);
	assert_eq!(list.linear_search(&3), Some(2));
}

#[test]
fn linear_search_not_found()
{
	let list = ArrayList::from_array([1, 2, 3]);
	assert_eq!(list.linear_search(&99), None);
}

#[test]
fn linear_search_empty_list()
{
	let list: ArrayList<i32> = ArrayList::new();
	assert_eq!(list.linear_search(&1), None);
}

#[test]
fn linear_search_works_on_unsorted_data()
{
	let list = ArrayList::from_array([30, 10, 20]);
	assert_eq!(list.linear_search(&10), Some(1));
}

#[test]
fn binary_search_finds_element()
{
	let list = ArrayList::from_array([1, 2, 3, 4, 5]);
	assert_eq!(list.binary_search(&3), Some(2));
}

#[test]
fn binary_search_first_element()
{
	let list = ArrayList::from_array([1, 2, 3, 4, 5]);
	assert_eq!(list.binary_search(&1), Some(0));
}

#[test]
fn binary_search_last_element()
{
	let list = ArrayList::from_array([1, 2, 3, 4, 5]);
	assert_eq!(list.binary_search(&5), Some(4));
}

#[test]
fn binary_search_not_found()
{
	let list = ArrayList::from_array([1, 2, 3, 4, 5]);
	assert_eq!(list.binary_search(&99), None);
}

#[test]
fn binary_search_single_element_found()
{
	let list = ArrayList::from_array([7]);
	assert_eq!(list.binary_search(&7), Some(0));
}

#[test]
fn binary_search_single_element_not_found_smaller()
{
	// target < only element: mid == 0, so `high = mid - 1` would underflow on
	// usize — this test guards against that regression.
	let list = ArrayList::from_array([7]);
	assert_eq!(list.binary_search(&1), None);
}

#[test]
fn binary_search_single_element_not_found_larger()
{
	// target > only element
	let list = ArrayList::from_array([7]);
	assert_eq!(list.binary_search(&99), None);
}

#[test]
fn binary_search_target_smaller_than_all_elements()
{
	// Forces the checked_sub(0) branch on the first iteration.
	let list = ArrayList::from_array([10, 20, 30, 40, 50]);
	assert_eq!(list.binary_search(&1), None);
}

#[test]
fn binary_search_on_sorted_result_of_sort()
{
	let mut list = ArrayList::from_array([5, 1, 3, 2, 4]);
	list.sort();
	// After sort: [1, 2, 3, 4, 5]
	assert_eq!(list.binary_search(&1), Some(0));
	assert_eq!(list.binary_search(&3), Some(2));
	assert_eq!(list.binary_search(&5), Some(4));
}
