//! Tests for all element-level mutation and access methods.

use arrlist::{
	arrlist::ArrayList,
	error::ListError,
};

#[test]
fn push_increments_len()
{
	let mut list = ArrayList::new();
	list.push(1).unwrap();
	list.push(2).unwrap();
	list.push(3).unwrap();
	assert_eq!(list.len(), 3);
}

#[test]
fn push_grows_from_zero()
{
	// Starting capacity is 0; first push must trigger a grow to 4.
	let mut list: ArrayList<i32> = ArrayList::new();
	list.push(42).unwrap();
	assert!(list.capacity() >= 1);
	assert_eq!(list.get(0), Some(&42));
}

#[test]
fn push_doubles_capacity_when_full()
{
	let mut list = ArrayList::with_capacity(2);
	list.push(1).unwrap();
	list.push(2).unwrap();
	assert_eq!(list.capacity(), 2);
	list.push(3).unwrap(); // triggers grow → capacity becomes 4
	assert_eq!(list.capacity(), 4);
}

#[test]
fn push_preserves_existing_elements_after_grow()
{
	let mut list = ArrayList::with_capacity(2);
	list.push(10).unwrap();
	list.push(20).unwrap();
	list.push(30).unwrap(); // forces realloc
	assert_eq!(list.get(0), Some(&10));
	assert_eq!(list.get(1), Some(&20));
	assert_eq!(list.get(2), Some(&30));
}

#[test]
fn pop_returns_last_element()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert_eq!(list.pop(), Some(3));
}

#[test]
fn pop_decrements_len()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	list.pop();
	assert_eq!(list.len(), 2);
}

#[test]
fn pop_empty_returns_none()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	assert_eq!(list.pop(), None);
}

#[test]
fn pop_until_empty()
{
	let mut list = ArrayList::from_array([1, 2]);
	list.pop();
	list.pop();
	assert!(list.is_empty());
	assert_eq!(list.pop(), None);
}

#[test]
fn pop_front_returns_first_element()
{
	let mut list = ArrayList::from_array([10, 20, 30]);
	assert_eq!(list.pop_front(), Some(10));
}

#[test]
fn pop_front_shifts_remaining_elements()
{
	let mut list = ArrayList::from_array([10, 20, 30]);
	list.pop_front();
	assert_eq!(list.get(0), Some(&20));
	assert_eq!(list.get(1), Some(&30));
	assert_eq!(list.len(), 2);
}

#[test]
fn pop_front_empty_returns_none()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	assert_eq!(list.pop_front(), None);
}

#[test]
fn pop_front_single_element()
{
	let mut list = ArrayList::from_array([42]);
	assert_eq!(list.pop_front(), Some(42));
	assert!(list.is_empty());
}

#[test]
fn get_valid_index()
{
	let list = ArrayList::from_array([5, 10, 15]);
	assert_eq!(list.get(0), Some(&5));
	assert_eq!(list.get(1), Some(&10));
	assert_eq!(list.get(2), Some(&15));
}

#[test]
fn get_out_of_bounds_returns_none()
{
	let list = ArrayList::from_array([1, 2, 3]);
	assert_eq!(list.get(3), None);
	assert_eq!(list.get(100), None);
}

#[test]
fn get_empty_list_returns_none()
{
	let list: ArrayList<i32> = ArrayList::new();
	assert_eq!(list.get(0), None);
}

#[test]
fn get_mut_allows_mutation()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	*list.get_mut(1).unwrap() = 99;
	assert_eq!(list.get(1), Some(&99));
}

#[test]
fn get_mut_out_of_bounds_returns_none()
{
	let mut list = ArrayList::from_array([1, 2]);
	assert!(list.get_mut(5).is_none());
}

#[test]
fn set_replaces_element()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	list.set(1, 42).unwrap();
	assert_eq!(list.get(1), Some(&42));
}

#[test]
fn set_does_not_change_len()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	list.set(0, 99).unwrap();
	assert_eq!(list.len(), 3);
}

#[test]
fn set_empty_list_returns_empty_list_error()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	assert!(matches!(list.set(0, 1), Err(ListError::EmptyList)));
}

#[test]
fn set_out_of_bounds_returns_error()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert!(matches!(
		list.set(10, 99),
		Err(ListError::OutOfBounds {
			idx: 10,
			limits: (0, 2)
		})
	));
}

#[test]
fn insert_at_front()
{
	let mut list = ArrayList::from_array([2, 3, 4]);
	list.insert(0, 1).unwrap();
	assert_eq!(list.get(0), Some(&1));
	assert_eq!(list.get(1), Some(&2));
	assert_eq!(list.len(), 4);
}

#[test]
fn insert_in_middle()
{
	let mut list = ArrayList::from_array([1, 3, 4]);
	list.insert(1, 2).unwrap();
	assert_eq!(list.get(0), Some(&1));
	assert_eq!(list.get(1), Some(&2));
	assert_eq!(list.get(2), Some(&3));
	assert_eq!(list.get(3), Some(&4));
}

#[test]
fn insert_at_end_is_like_push()
{
	let mut list = ArrayList::from_array([1, 2]);
	list.insert(2, 3).unwrap();
	assert_eq!(list.get(2), Some(&3));
	assert_eq!(list.len(), 3);
}

#[test]
fn insert_empty_list_out_of_bounds()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	// idx == 0 on empty list is allowed (acts like push), idx > 0 is an error
	assert!(matches!(list.insert(1, 99), Err(ListError::EmptyList)));
}

#[test]
fn insert_beyond_len_returns_error()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert!(matches!(
		list.insert(10, 99),
		Err(ListError::OutOfBounds { .. })
	));
}

#[test]
fn remove_returns_correct_value()
{
	let mut list = ArrayList::from_array([10, 20, 30]);
	assert_eq!(list.remove(1).unwrap(), 20);
}

#[test]
fn remove_shifts_elements_left()
{
	let mut list = ArrayList::from_array([10, 20, 30]);
	list.remove(0).unwrap();
	assert_eq!(list.get(0), Some(&20));
	assert_eq!(list.get(1), Some(&30));
	assert_eq!(list.len(), 2);
}

#[test]
fn remove_last_element()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	assert_eq!(list.remove(2).unwrap(), 3);
	assert_eq!(list.len(), 2);
}

#[test]
fn remove_empty_list_returns_empty_list_error()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	assert!(matches!(list.remove(0), Err(ListError::EmptyList)));
}

#[test]
fn remove_out_of_bounds_returns_error()
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
fn clear_empties_list()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	list.clear();
	assert!(list.is_empty());
	assert_eq!(list.len(), 0);
}

#[test]
fn clear_retains_capacity()
{
	let mut list = ArrayList::with_capacity(16);
	list.push(1).unwrap();
	list.push(2).unwrap();
	list.clear();
	assert_eq!(list.capacity(), 16);
}

#[test]
fn clear_allows_reuse()
{
	let mut list = ArrayList::from_array([1, 2, 3]);
	list.clear();
	list.push(42).unwrap();
	assert_eq!(list.len(), 1);
	assert_eq!(list.get(0), Some(&42));
}

#[test]
fn clear_on_empty_list_is_a_noop()
{
	let mut list: ArrayList<i32> = ArrayList::new();
	list.clear(); // must not panic
	assert!(list.is_empty());
}

/// Confirms that `Drop` is called on every element when the list is dropped,
/// using a counter tracked through a shared reference.
#[test]
fn drop_runs_element_destructors()
{
	use core::cell::Cell;

	struct DropCounter<'a>
	{
		count: &'a Cell<u32>,
	}

	impl Drop for DropCounter<'_>
	{
		fn drop(&mut self)
		{
			self.count.set(self.count.get() + 1);
		}
	}

	let count = Cell::new(0u32);
	{
		let mut list = ArrayList::new();
		list.push(DropCounter { count: &count }).unwrap();
		list.push(DropCounter { count: &count }).unwrap();
		list.push(DropCounter { count: &count }).unwrap();
	} // list dropped here
	assert_eq!(count.get(), 3);
}

/// Confirms that `set` properly drops the value it overwrites.
/// Without the explicit `assume_init_drop` call the old value would leak.
#[test]
fn set_drops_old_value()
{
	use core::cell::Cell;

	struct DropCounter<'a>
	{
		count: &'a Cell<u32>,
	}

	impl Drop for DropCounter<'_>
	{
		fn drop(&mut self)
		{
			self.count.set(self.count.get() + 1);
		}
	}

	let count = Cell::new(0u32);
	{
		let mut list = ArrayList::new();
		list.push(DropCounter { count: &count }).unwrap();
		// Overwrite slot 0 — the original DropCounter must be dropped here.
		list.set(0, DropCounter { count: &count }).unwrap();
		assert_eq!(count.get(), 1); // old value dropped by set
	} // new value dropped here
	assert_eq!(count.get(), 2);
}
