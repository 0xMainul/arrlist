//! Core [`ArrayList<T>`] data structure.
//!
//! This module provides the main [`ArrayList<T>`] type along with its three iterator
//! types — [`IntoIter`], [`Iter`], and [`IterMut`] — and all associated method
//! implementations.
//!
//! # Memory Model
//!
//! [`ArrayList<T>`] stores elements in a heap-allocated `Box<[MaybeUninit<T>]>`. Only
//! the first `len` slots are initialised at any given time; slots in the range
//! `[len, capacity]` are logically uninitialised and are never read as `T`. This
//! means:
//!
//! - `T` is **never required to implement [`Default`]**.
//! - Drop glue is only run on the initialised prefix — uninitialised slots are
//!   skipped in both [`Drop`] and [`clear`](ArrayList::clear).
//! - All `unsafe` code in this module relies on the invariant that `self.len`
//!   accurately tracks how many slots are initialised.
//!
//! # Growth Strategy
//!
//! When [`push`](ArrayList::push) or [`insert`](ArrayList::insert) would exceed the
//! current capacity, the list calls the private `grow` method,
//! which doubles the capacity (starting from 4 for a fresh list). This gives
//! **amortised O(1)** cost per element appended.
//!
//! # Iterator Types
//!
//! | Type | Created by | Yields |
//! |------|-----------|--------|
//! | [`IntoIter<T>`] | `list.into_iter()` | `T` (owned) |
//! | [`Iter<'a, T>`] | `list.iter()` / `&list` | `&'a T` |
//! | [`IterMut<'a, T>`] | `list.iter_mut()` / `&mut list` | `&'a mut T` |

#![allow(unused)]

extern crate alloc;
use alloc::{
	boxed::Box,
	vec::Vec,
};

use crate::error::ListError;

use core::{
	fmt::{
		Debug,
		Display,
	},
	mem::MaybeUninit,
	ptr,
	slice,
};

/// A heap-allocated, dynamically-sized list backed by a contiguous block of memory.
///
/// `ArrayList<T>` stores elements in a boxed slice of [`MaybeUninit<T>`] slots, tracking
/// how many are currently initialised via `len`. When the list is full, it grows by
/// doubling its capacity (starting from 4), similar to `Vec<T>`.
///
/// This type is `no_std` compatible and requires only `alloc`.
///
/// # Type Parameter
///
/// - `T` — the element type. No bounds are required unless a specific method needs them
///   (e.g., `Ord` for [`sort`](ArrayList::sort) or `Clone` for [`from_slice`](ArrayList::from_slice)).
///
/// # Memory Layout
///
/// Elements are stored contiguously in memory. Only the first `len` slots are
/// initialised; slots in `[len, capacity)` are logically uninitialised and must not
/// be read.
///
/// # Examples
///
/// ```rust
/// use arrlist::arrlist::ArrayList;
///
/// let mut list = ArrayList::with_capacity(4);
/// list.push(1);
/// list.push(2);
/// list.push(3);
///
/// for val in &list
/// {
/// 	println!("{val}");
/// }
/// ```
pub struct ArrayList<T>
{
	/// The backing storage. Slots `0..len` are initialised; the rest are not.
	data: Box<[MaybeUninit<T>]>,
	/// Number of currently initialised (live) elements.
	len: usize,
	/// Total number of slots allocated in `data`.
	capacity: usize,
}

impl<T> ArrayList<T>
{
	/// Creates a new, empty `ArrayList` with no allocated storage.
	///
	/// No heap allocation occurs until the first element is pushed.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list: ArrayList<i32> = ArrayList::new();
	/// assert!(list.is_empty());
	/// assert_eq!(list.capacity(), 0);
	/// ```
	pub fn new() -> Self
	{
		Self {
			data: Box::new([]),
			len: 0,
			capacity: 0,
		}
	}

	/// Creates a new, empty `ArrayList` pre-allocated to hold at least `capacity` elements.
	///
	/// If `capacity` is 0, this is equivalent to [`new`](ArrayList::new).
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list: ArrayList<u8> = ArrayList::with_capacity(16);
	/// assert_eq!(list.capacity(), 16);
	/// assert_eq!(list.len(), 0);
	/// ```
	pub fn with_capacity(capacity: usize) -> Self
	{
		if capacity == 0
		{
			return Self::new();
		}

		Self {
			data: (0..capacity)
				.map(|_| MaybeUninit::uninit())
				.collect::<Vec<_>>()
				.into_boxed_slice(),
			len: 0,
			capacity,
		}
	}

	/// Returns the number of elements currently in the list.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list = ArrayList::from_array([10, 20, 30]);
	/// assert_eq!(list.len(), 3);
	/// ```
	pub fn len(&self) -> usize
	{
		self.len
	}

	/// Returns the total number of elements the list can hold without reallocating.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list: ArrayList<i32> = ArrayList::with_capacity(8);
	/// assert_eq!(list.capacity(), 8);
	/// ```
	pub fn capacity(&self) -> usize
	{
		self.capacity
	}

	/// Returns `true` if the list contains no elements.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list: ArrayList<i32> = ArrayList::new();
	/// assert!(list.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool
	{
		self.len == 0
	}

	/// Appends `val` to the back of the list.
	///
	/// If the list is at capacity, it will grow automatically (capacity doubles,
	/// starting from 4).
	///
	/// # Errors
	///
	/// Returns [`ListError::CapacityOverflow`] if the list needs to grow but
	/// doubling the current capacity would overflow `usize`.
	///
	/// # Complexity
	///
	/// Amortised O(1).
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::new();
	/// list.push(1).unwrap();
	/// list.push(2).unwrap();
	/// assert_eq!(list.len(), 2);
	/// ```
	pub fn push(&mut self, val: T) -> Result<(), ListError>
	{
		// Grow the backing storage if there is no room for the new element.
		if self.capacity == self.len
		{
			self.grow()?;
		}

		self.data[self.len].write(val);
		self.len += 1;
		Ok(())
	}

	/// Removes and returns the last element, or `None` if the list is empty.
	///
	/// # Complexity
	///
	/// O(1).
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([1, 2, 3]);
	/// assert_eq!(list.pop(), Some(3));
	/// assert_eq!(list.len(), 2);
	/// ```
	pub fn pop(&mut self) -> Option<T>
	{
		if self.len == 0
		{
			return None;
		}

		self.len -= 1;
		// SAFETY: The slot at `self.len` was initialised by a prior `push` or `insert`,
		// and we have just decremented `len` so it will no longer be considered live.
		unsafe { Some(self.data[self.len].assume_init_read()) }
	}

	/// Removes and returns the first element, shifting all remaining elements left.
	///
	/// Returns `None` if the list is empty.
	///
	/// # Complexity
	///
	/// O(n) — every remaining element is moved one position to the left.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([10, 20, 30]);
	/// assert_eq!(list.pop_front(), Some(10));
	/// assert_eq!(list.get(0), Some(&20));
	/// ```
	pub fn pop_front(&mut self) -> Option<T>
	{
		if self.len == 0
		{
			return None;
		}

		unsafe {
			// Read the first element before we shift everything over it.
			let val = self.data[0].assume_init_read();

			// Shift elements [1, len) one slot to the left.
			ptr::copy(
				self.data.as_ptr().add(1),
				self.data.as_mut_ptr(),
				self.len - 1,
			);

			self.len -= 1;
			// Mark the now-unused trailing slot as uninitialised.
			self.data[self.len] = MaybeUninit::uninit();

			Some(val)
		}
	}

	/// Returns a shared reference to the element at `idx`, or `None` if out of bounds.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list = ArrayList::from_array(['a', 'b', 'c']);
	/// assert_eq!(list.get(1), Some(&'b'));
	/// assert_eq!(list.get(99), None);
	/// ```
	pub fn get(&self, idx: usize) -> Option<&T>
	{
		if idx >= self.len
		{
			return None;
		}

		// SAFETY: `idx < self.len`, so this slot is guaranteed to be initialised.
		unsafe { Some(self.data[idx].assume_init_ref()) }
	}

	/// Returns a mutable reference to the element at `idx`, or `None` if out of bounds.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([1, 2, 3]);
	/// if let Some(val) = list.get_mut(0)
	/// {
	/// 	*val = 100;
	/// }
	/// assert_eq!(list.get(0), Some(&100));
	/// ```
	pub fn get_mut(&mut self, idx: usize) -> Option<&mut T>
	{
		if idx >= self.len
		{
			return None;
		}

		// SAFETY: `idx < self.len`, so this slot is guaranteed to be initialised.
		unsafe { Some(self.data[idx].assume_init_mut()) }
	}

	/// Replaces the element at `idx` with `val`.
	///
	/// Returns [`ListError::EmptyList`] if the list is empty, or
	/// [`ListError::OutOfBounds`] if `idx >= len`.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([1, 2, 3]);
	/// list.set(1, 99).unwrap();
	/// assert_eq!(list.get(1), Some(&99));
	/// ```
	pub fn set(&mut self, idx: usize, val: T) -> Result<(), ListError>
	{
		if idx >= self.len
		{
			// Distinguish between "list is empty" and "index is just too large".
			match self.is_empty()
			{
				true =>
				{
					return Err(ListError::EmptyList);
				},
				false =>
				{
					return Err(ListError::OutOfBounds {
						idx,
						limits: (0, self.len - 1),
					});
				},
			}
		}

		// SAFETY: `idx < self.len`, so this slot is initialised. We must drop
		// the old value explicitly before overwriting, otherwise it leaks.
		unsafe { self.data[idx].assume_init_drop() };
		self.data[idx] = MaybeUninit::new(val);
		Ok(())
	}

	/// Inserts `val` at position `idx`, shifting all elements at and after `idx` one
	/// position to the right.
	///
	/// `idx` may equal `self.len()` to append at the back (equivalent to [`push`](ArrayList::push)).
	///
	/// Returns [`ListError::EmptyList`] if the list is empty and `idx > 0`, or
	/// [`ListError::OutOfBounds`] if `idx > len`.
	///
	/// # Complexity
	///
	/// O(n) — elements after `idx` must be shifted right.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([1, 3, 4]);
	/// list.insert(1, 2).unwrap();
	/// assert_eq!(list.get(1), Some(&2));
	/// assert_eq!(list.get(2), Some(&3));
	/// ```
	pub fn insert(&mut self, idx: usize, val: T) -> Result<(), ListError>
	{
		if idx > self.len
		{
			match self.is_empty()
			{
				true =>
				{
					return Err(ListError::EmptyList);
				},
				false =>
				{
					return Err(ListError::OutOfBounds {
						idx,
						limits: (0, self.len - 1),
					});
				},
			}
		}

		// Grow before shifting to ensure the extra slot exists.
		if self.len == self.capacity
		{
			self.grow()?;
		}

		unsafe {
			// Shift elements [idx, len) one slot to the right to open a gap at `idx`.
			ptr::copy(
				self.data.as_ptr().add(idx),
				self.data.as_mut_ptr().add(idx + 1),
				self.len - idx,
			);
		}
		self.data[idx] = MaybeUninit::new(val);
		self.len += 1;

		Ok(())
	}

	/// Removes and returns the element at `idx`, shifting all subsequent elements
	/// one position to the left.
	///
	/// Returns [`ListError::EmptyList`] if the list is empty, or
	/// [`ListError::OutOfBounds`] if `idx >= len`.
	///
	/// # Complexity
	///
	/// O(n) — elements after `idx` must be shifted left.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([10, 20, 30]);
	/// assert_eq!(list.remove(1).unwrap(), 20);
	/// assert_eq!(list.len(), 2);
	/// assert_eq!(list.get(1), Some(&30));
	/// ```
	pub fn remove(&mut self, idx: usize) -> Result<T, ListError>
	{
		if idx >= self.len
		{
			match self.is_empty()
			{
				true =>
				{
					return Err(ListError::EmptyList);
				},
				false =>
				{
					return Err(ListError::OutOfBounds {
						idx,
						limits: (0, self.len - 1),
					});
				},
			}
		}

		unsafe {
			// Read the value out before we overwrite the slot.
			let val = self.data[idx].assume_init_read();

			// Shift elements [idx+1, len) one slot to the left.
			ptr::copy(
				self.data.as_ptr().add(idx + 1),
				self.data.as_mut_ptr().add(idx),
				self.len - idx - 1,
			);

			self.len -= 1;
			// Mark the trailing slot as uninitialised.
			self.data[self.len] = MaybeUninit::uninit();
			Ok(val)
		}
	}

	/// Grows the backing buffer by doubling its capacity.
	///
	/// If the current capacity is 0, the new capacity is set to 4.
	/// Existing initialised elements are copied into the new allocation via
	/// [`ptr::copy_nonoverlapping`].
	///
	/// # Errors
	///
	/// Returns [`ListError::CapacityOverflow`] if doubling the current capacity
	/// would overflow `usize`.
	fn grow(&mut self) -> Result<(), ListError>
	{
		let new_capacity = if self.capacity == 0
		{
			// Start small to avoid wasting memory on tiny lists.
			4
		}
		else
		{
			// Double the capacity to achieve amortised O(1) push.
			// Return CapacityOverflow instead of panicking so callers can handle it.
			self.capacity
				.checked_mul(2)
				.ok_or(ListError::CapacityOverflow)?
		};

		let mut new_data: Box<[MaybeUninit<T>]> = (0..new_capacity)
			.map(|_| MaybeUninit::uninit())
			.collect::<Vec<_>>()
			.into_boxed_slice();

		unsafe {
			// Copy all initialised elements into the new buffer.
			// The old buffer is then overwritten by the assignment below, so we must
			// NOT drop the elements from it — `MaybeUninit` ensures that.
			ptr::copy_nonoverlapping(self.data.as_ptr(), new_data.as_mut_ptr(), self.len);
		}

		self.data = new_data;
		self.capacity = new_capacity;
		Ok(())
	}

	/// Removes all elements from the list, dropping each one in order.
	///
	/// After this call, `len` is 0. The allocated capacity is retained.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([1, 2, 3]);
	/// list.clear();
	/// assert!(list.is_empty());
	/// ```
	pub fn clear(&mut self)
	{
		// Drop every initialised element in order.
		for idx in 0..self.len
		{
			unsafe { self.data[idx].assume_init_drop() };
		}
		self.len = 0;
	}
}

impl<T> ArrayList<T>
{
	/// Creates an `ArrayList` from an existing `Vec<T>` without cloning.
	///
	/// The underlying buffer of the `Vec` is reused directly. Ownership is transferred
	/// so no allocation occurs.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let v = vec![1, 2, 3];
	/// let list = ArrayList::from_vec(v);
	/// assert_eq!(list.len(), 3);
	/// ```
	pub fn from_vec(vec: Vec<T>) -> Self
	{
		let len = vec.len();
		let capacity = vec.capacity();

		// Transmute the Vec's buffer into a `Box<[MaybeUninit<T>]>` without
		// copying or running any destructors. `ManuallyDrop` prevents the
		// original `Vec` from freeing the buffer when it goes out of scope.
		let data = unsafe {
			let mut vec = core::mem::ManuallyDrop::new(vec);
			let ptr = vec.as_mut_ptr() as *mut MaybeUninit<T>;
			let cap = vec.capacity();

			// We build the box over the *full* allocated capacity (not just `len`)
			// so that the allocator receives the correct size when the box is freed.
			// `self.len` separately tracks how many of those slots are initialised.
			Vec::from_raw_parts(ptr, cap, cap).into_boxed_slice()
		};

		Self {
			data,
			len,
			capacity,
		}
	}

	/// Creates an `ArrayList` from a fixed-size array, consuming it.
	///
	/// Elements are moved into the list one by one. No cloning occurs.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list = ArrayList::from_array([10, 20, 30]);
	/// assert_eq!(list.len(), 3);
	/// assert_eq!(list.get(2), Some(&30));
	/// ```
	pub fn from_array<const N: usize>(arr: [T; N]) -> Self
	{
		let mut list = Self::with_capacity(N);

		for item in arr
		{
			// SAFETY: capacity is pre-allocated to exactly N, so grow is never
			// triggered and push cannot return Err.
			list.push(item)
				.expect("push failed in from_array: pre-allocated capacity exhausted");
		}

		list
	}

	/// Creates an `ArrayList` by cloning each element from a slice.
	///
	/// Requires `T: Clone`.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let src = [1, 2, 3];
	/// let list = ArrayList::from_slice(&src);
	/// assert_eq!(list.len(), 3);
	/// ```
	pub fn from_slice(slice: &[T]) -> Self
	where
		T: Clone,
	{
		let mut list = Self::with_capacity(slice.len());

		for item in slice
		{
			// SAFETY: capacity is pre-allocated to slice.len(), so grow is never
			// triggered and push cannot return Err.
			list.push(item.clone())
				.expect("push failed in from_slice: pre-allocated capacity exhausted");
		}

		list
	}
}

impl<T> ArrayList<T>
{
	/// Reverses the order of elements in place.
	///
	/// Uses a two-pointer swap approach. Does nothing if the list has 0 or 1 element.
	///
	/// # Complexity
	///
	/// O(n).
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([1, 2, 3, 4]);
	/// list.reverse();
	/// assert_eq!(list.get(0), Some(&4));
	/// assert_eq!(list.get(3), Some(&1));
	/// ```
	pub fn reverse(&mut self)
	{
		if self.len <= 1
		{
			return;
		}

		let mut left = 0;
		let mut right = self.len - 1;

		// Walk inward from both ends, swapping pairs.
		while left < right
		{
			unsafe {
				ptr::swap(
					self.data.as_mut_ptr().add(left),
					self.data.as_mut_ptr().add(right),
				);
			}
			left += 1;
			right -= 1;
		}
	}

	/// Sorts the list in ascending order using bubble sort.
	///
	/// Requires `T: Ord`. Does nothing if the list has 0 or 1 element.
	///
	/// # Complexity
	///
	/// O(n²) in the worst and average case. Suitable for small lists or nearly-sorted
	/// data; consider extracting to a `Vec` and using `slice::sort` for larger inputs.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([3, 1, 4, 1, 5]);
	/// list.sort();
	/// assert_eq!(list.get(0), Some(&1));
	/// assert_eq!(list.get(4), Some(&5));
	/// ```
	pub fn sort(&mut self)
	where
		T: Ord,
	{
		match self.len <= 1
		{
			true => (),
			false =>
			{
				// Outer pass: each iteration guarantees the largest unsorted element
				// has bubbled to its final position at the end of the unsorted region.
				for idx in 0..self.len
				{
					for jdx in 0..self.len - idx - 1
					{
						unsafe {
							if self.data[jdx].assume_init_ref()
								> self.data[jdx + 1].assume_init_ref()
							{
								// Swap adjacent elements that are out of order.
								ptr::swap(
									self.data.as_mut_ptr().add(jdx),
									self.data.as_mut_ptr().add(jdx + 1),
								);
							}
						}
					}
				}
			},
		}
	}

	/// Returns the index of the first element equal to `target`, or `None`.
	///
	/// Requires `T: PartialEq`. Scans from index 0 to `len - 1`.
	///
	/// # Complexity
	///
	/// O(n).
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list = ArrayList::from_array([10, 20, 30]);
	/// assert_eq!(list.linear_search(&20), Some(1));
	/// assert_eq!(list.linear_search(&99), None);
	/// ```
	pub fn linear_search(&self, target: &T) -> Option<usize>
	where
		T: PartialEq,
	{
		for idx in 0..self.len
		{
			unsafe {
				if target == self.data[idx].assume_init_ref()
				{
					return Some(idx);
				}
			}
		}
		None
	}

	/// Returns the index of an element equal to `target` using binary search, or `None`.
	///
	/// Requires `T: Ord`. **The list must be sorted in ascending order** before calling
	/// this method; results are unspecified on unsorted data.
	///
	/// # Complexity
	///
	/// O(log n).
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([1, 3, 5, 7, 9]);
	/// assert_eq!(list.binary_search(&5), Some(2));
	/// assert_eq!(list.binary_search(&4), None);
	/// ```
	pub fn binary_search(&self, target: &T) -> Option<usize>
	where
		T: Ord,
	{
		// Guard against empty list: `self.len - 1` would underflow.
		if self.is_empty()
		{
			return None;
		}

		let mut low = 0usize;
		let mut high = self.len - 1;

		while low <= high
		{
			// Use `low + (high - low) / 2` to avoid overflow.
			let mid = low + (high - low) / 2;

			unsafe {
				if self.data[mid].assume_init_ref() == target
				{
					return Some(mid);
				}
				else if self.data[mid].assume_init_ref() < target
				{
					low = mid + 1;
				}
				else
				{
					// `mid` is 0 when the target is smaller than every element.
					// Subtracting 1 from a `usize` of 0 would panic in debug or
					// wrap in release, so we use `checked_sub` and bail out.
					match mid.checked_sub(1)
					{
						Some(val) => high = val,
						None => return None,
					}
				}
			}
		}
		None
	}
}

impl<T> From<Vec<T>> for ArrayList<T>
{
	/// Converts a `Vec<T>` into an `ArrayList<T>` without copying.
	fn from(vec: Vec<T>) -> Self
	{
		Self::from_vec(vec)
	}
}

impl<T, const N: usize> From<[T; N]> for ArrayList<T>
{
	/// Converts a fixed-size array `[T; N]` into an `ArrayList<T>`, consuming it.
	fn from(arr: [T; N]) -> Self
	{
		Self::from_array(arr)
	}
}

impl<T> Default for ArrayList<T>
{
	/// Creates an empty `ArrayList` with no heap allocation.
	///
	/// This is identical to calling [`ArrayList::new()`] and is provided so that
	/// `ArrayList<T>` can be used anywhere a `Default` bound is required — for
	/// example, as a field inside a `#[derive(Default)]` struct, or with
	/// [`Option::unwrap_or_default`].
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list: ArrayList<i32> = ArrayList::default();
	/// assert!(list.is_empty());
	/// assert_eq!(list.capacity(), 0);
	/// ```
	///
	/// Using `Default` inside another struct:
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// #[derive(Default)]
	/// struct State
	/// {
	/// 	items: ArrayList<String>,
	/// }
	///
	/// let state = State::default();
	/// assert!(state.items.is_empty());
	/// ```
	fn default() -> Self
	{
		Self::new()
	}
}

impl<T: Debug> Display for ArrayList<T>
{
	/// Formats the list as `[elem0, elem1, …]` using each element's `Debug` representation.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list = ArrayList::from_array([1, 2, 3]);
	/// assert_eq!(format!("{list}"), "[1, 2, 3]");
	/// ```
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		write!(f, "[");

		for idx in 0..self.len
		{
			if idx > 0
			{
				write!(f, ", ");
			}

			unsafe {
				write!(f, "{:?}", self.data[idx].assume_init_ref());
			}
		}

		write!(f, "]")
	}
}

impl<T> Drop for ArrayList<T>
{
	/// Drops all initialised elements when the `ArrayList` is destroyed.
	///
	/// Slots beyond `len` are uninitialised and are intentionally skipped.
	fn drop(&mut self)
	{
		for idx in 0..self.len
		{
			unsafe {
				self.data[idx].assume_init_drop();
			}
		}
	}
}

/// An owning iterator over an [`ArrayList<T>`].
///
/// Elements are yielded front-to-back by repeatedly calling [`pop_front`](ArrayList::pop_front).
/// Created by [`ArrayList::into_iter`] via the [`IntoIterator`] implementation.
pub struct IntoIter<T>
{
	arr: ArrayList<T>,
}

impl<T> Iterator for IntoIter<T>
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item>
	{
		self.arr.pop_front()
	}
}

impl<T> IntoIterator for ArrayList<T>
{
	type IntoIter = IntoIter<T>;
	type Item = T;

	fn into_iter(self) -> Self::IntoIter
	{
		IntoIter { arr: self }
	}
}

/// A borrowing iterator over an [`ArrayList<T>`] that yields shared references.
///
/// Created by [`ArrayList::iter`] or by iterating over `&ArrayList<T>`.
pub struct Iter<'a, T>
{
	arr: &'a ArrayList<T>,
	/// Current position; incremented on each call to [`next`](Iterator::next).
	index: usize,
}

impl<T> ArrayList<T>
{
	/// Returns an iterator over shared references to the elements of the list.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let list = ArrayList::from_array([1, 2, 3]);
	/// let collected: Vec<_> = list.iter().copied().collect();
	/// assert_eq!(collected, [1, 2, 3]);
	/// ```
	pub fn iter(&self) -> Iter<'_, T>
	{
		Iter {
			arr: self,
			index: 0,
		}
	}
}

impl<'a, T> Iterator for Iter<'a, T>
{
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.index >= self.arr.len()
		{
			None
		}
		else
		{
			let item = unsafe {
				let ptr = self.arr.data.as_ptr().add(self.index);
				(*ptr).assume_init_ref()
			};
			self.index += 1;
			Some(item)
		}
	}
}

/// A mutably-borrowing iterator over an [`ArrayList<T>`] that yields exclusive references.
///
/// Created by [`ArrayList::iter_mut`] or by iterating over `&mut ArrayList<T>`.
pub struct IterMut<'a, T>
{
	arr: &'a mut ArrayList<T>,
	/// Current position; incremented on each call to [`next`](Iterator::next).
	index: usize,
}

impl<T> ArrayList<T>
{
	/// Returns an iterator over mutable references to the elements of the list.
	///
	/// # Examples
	///
	/// ```rust
	/// use arrlist::arrlist::ArrayList;
	///
	/// let mut list = ArrayList::from_array([1, 2, 3]);
	/// for val in list.iter_mut()
	/// {
	/// 	*val *= 2;
	/// }
	/// assert_eq!(list.get(0), Some(&2));
	/// assert_eq!(list.get(2), Some(&6));
	/// ```
	pub fn iter_mut(&mut self) -> IterMut<'_, T>
	{
		IterMut {
			arr: self,
			index: 0,
		}
	}
}

impl<'a, T> Iterator for IterMut<'a, T>
{
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item>
	{
		if self.index >= self.arr.len()
		{
			None
		}
		else
		{
			let item = unsafe {
				let ptr = self.arr.data.as_mut_ptr().add(self.index);
				(*ptr).assume_init_mut()
			};
			self.index += 1;
			Some(item)
		}
	}
}

impl<'a, T> IntoIterator for &'a ArrayList<T>
{
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;

	fn into_iter(self) -> Self::IntoIter
	{
		self.iter()
	}
}

impl<'a, T> IntoIterator for &'a mut ArrayList<T>
{
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;

	fn into_iter(self) -> Self::IntoIter
	{
		self.iter_mut()
	}
}
