//! Error types for fallible [`ArrayList`](crate::ArrayList) operations.
//!
//! The single public type in this module is [`ListError`], returned by methods that
//! accept an index or mutate the list's capacity.
//!
//! # When Each Variant Is Returned
//!
//! | Situation | Variant |
//! |-----------|---------|
//! | `idx >= len` and `len == 0` | [`ListError::EmptyList`] |
//! | `idx >= len` and `len > 0` | [`ListError::OutOfBounds`] |
//! | doubling capacity would overflow `usize` | [`ListError::CapacityOverflow`] |
//!
//! # Example
//!
//! ```rust
//! use arrlist::{
//! 	ArrayList,
//! 	error::ListError,
//! };
//!
//! let mut list: ArrayList<i32> = ArrayList::new();
//!
//! // Empty list — EmptyList is returned regardless of which index is used.
//! assert!(matches!(list.remove(0), Err(ListError::EmptyList)));
//!
//! list.push(10);
//! list.push(20);
//!
//! // Non-empty list, but index 5 is out of range.
//! assert!(matches!(
//! 	list.remove(5),
//! 	Err(ListError::OutOfBounds {
//! 		idx: 5,
//! 		limits: (0, 1)
//! 	})
//! ));
//! ```

use thiserror_no_std::Error;

/// Errors that can occur during [`ArrayList`](crate::ArrayList) operations.
///
/// This enum covers three failure modes: accessing an out-of-range index, operating on
/// an empty list, and exhausting the addressable capacity of the allocator.
#[derive(Debug, Error)]
pub enum ListError
{
	/// The provided index is outside the valid range of the list.
	///
	/// Contains the attempted index and a tuple `(min, max)` representing the valid
	/// inclusive bounds at the time of the access.
	///
	/// # Example
	///
	/// ```
	/// use arrlist::{
	/// 	ArrayList,
	/// 	error::ListError,
	/// };
	///
	/// let mut list = ArrayList::from_array([1, 2, 3]);
	/// let err = list.set(10, 99).unwrap_err();
	/// assert!(matches!(
	/// 	err,
	/// 	ListError::OutOfBounds {
	/// 		idx: 10,
	/// 		limits: (0, 2)
	/// 	}
	/// ));
	/// ```
	#[error("invalid index {idx}, expected at least {} and at most {}", .limits.0, .limits.1)]
	OutOfBounds
	{
		/// The index that was attempted.
		idx: usize,
		/// The valid inclusive index range `(min, max)`.
		limits: (usize, usize),
	},

	/// An operation was attempted on an empty list.
	///
	/// Returned when an index-based operation (e.g., [`set`](crate::ArrayList::set),
	/// [`insert`](crate::ArrayList::insert), [`remove`](crate::ArrayList::remove))
	/// is called but the list contains no elements.
	///
	/// # Example
	///
	/// ```
	/// use arrlist::{
	/// 	ArrayList,
	/// 	error::ListError,
	/// };
	///
	/// let mut list: ArrayList<i32> = ArrayList::new();
	/// let err = list.remove(0).unwrap_err();
	/// assert!(matches!(err, ListError::EmptyList));
	/// ```
	#[error("the list is empty")]
	EmptyList,

	/// The backing buffer cannot grow because doubling the capacity would overflow
	/// `usize`.
	///
	/// On 64-bit systems this requires a list already holding roughly 9 × 10¹⁸
	/// elements, so it is practically unreachable in normal use. On 32-bit targets
	/// the limit is ~2 billion elements.
	///
	/// Returned by [`push`](crate::ArrayList::push) and
	/// [`insert`](crate::ArrayList::insert) when a reallocation is
	/// required but the new capacity cannot be represented as a `usize`.
	///
	/// # Example
	///
	/// ```rust
	/// // Constructing a list this large is impractical in a real test, but the
	/// // variant can be matched and inspected like any other error:
	/// use arrlist::error::ListError;
	/// let err = ListError::CapacityOverflow;
	/// assert_eq!(
	/// 	err.to_string(),
	/// 	"capacity overflow: cannot grow the list any further"
	/// );
	/// ```
	#[error("capacity overflow: cannot grow the list any further")]
	CapacityOverflow,
}
