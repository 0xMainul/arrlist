//! # arrlist
//!
//! A generic, heap-allocated dynamic array for `no_std` environments.
//!
//! [`ArrayList<T>`](crate::arrlist::ArrayList) is a contiguous, growable collection backed by a boxed slice of
//! [`MaybeUninit<T>`](core::mem::MaybeUninit). It provides O(1) amortized push/pop at the
//! back, O(n) insertion and removal at arbitrary positions, and a full set of
//! iterator types — all without requiring the standard library.
//!
//! ## Features
//!
//! - `no_std` compatible (requires `alloc`)
//! - Manual memory management via [`MaybeUninit`](core::mem::MaybeUninit) — no unnecessary [`Default`] bounds
//! - Amortized O(1) [`push`](arrlist::ArrayList::push) and [`pop`](arrlist::ArrayList::pop)
//! - O(n) [`insert`](arrlist::ArrayList::insert), [`remove`](arrlist::ArrayList::remove), and [`pop_front`](arrlist::ArrayList::pop_front)
//! - Conversion from `Vec<T>`, `[T; N]`, and `&[T]`
//! - Owned, shared, and mutable iterators
//! - Built-in [`sort`](arrlist::ArrayList::sort) (bubble sort), [`reverse`](arrlist::ArrayList::reverse),
//!   [`linear_search`](arrlist::ArrayList::linear_search), and [`binary_search`](arrlist::ArrayList::binary_search)
//!
//! ## Quick Start
//!
//! ```rust
//! use arrlist::{arrlist, ArrayList};
//!
//! // Using the macro — just like vec![]
//! let list = arrlist![1, 2, 3];
//! assert_eq!(list.len(), 3);
//! assert_eq!(list.get(1), Some(&2));
//!
//! // Or create an empty list with a given capacity: arrlist![0; 8]
//! let list: ArrayList<i32> = arrlist![0; 8];
//! assert_eq!(list.capacity(), 8);
//! assert_eq!(list.len(), 8);
//! ```
#![allow(clippy::tabs_in_doc_comments)]
#![no_std]

pub mod arrlist;
pub mod error;

/// Creates an [`ArrayList`](crate::arrlist::ArrayList) with the given elements,
/// mirroring the syntax of the standard `vec!` macro.
///
/// # Forms
///
/// ## List of elements
///
/// ```rust
/// use arrlist::arrlist;
///
/// let list = arrlist![10, 20, 30];
/// assert_eq!(list.len(), 3);
/// assert_eq!(list.get(0), Some(&10));
/// assert_eq!(list.get(2), Some(&30));
/// ```
///
/// ## Repeated element
///
/// `arrlist![value; count]` creates a list of `count` elements all cloned from `value`.
/// Requires the element type to implement [`Clone`].
///
/// ```rust
/// use arrlist::arrlist;
///
/// let list = arrlist![0_i32; 5];
/// assert_eq!(list.len(), 5);
/// assert_eq!(list.get(4), Some(&0));
/// ```
///
/// ## Empty list
///
/// ```rust
/// use arrlist::{
/// 	ArrayList,
/// 	arrlist,
/// };
///
/// let list: ArrayList<i32> = arrlist![];
/// assert!(list.is_empty());
/// ```
#[macro_export]
macro_rules! arrlist {
    // arrlist![] — empty list
    () => {
        $crate::arrlist::ArrayList::new()
    };

    // arrlist![val; n] — n copies of val
    ($elem:expr; $n:expr) => {
        {
            let mut list = $crate::arrlist::ArrayList::with_capacity($n);
            let mut remaining = $n;
            while remaining > 1 {
                list.push(::core::clone::Clone::clone(&$elem)).expect("arrlist![val; n]: capacity overflow");
                remaining -= 1;
            }
            if $n > 0 {
                // Move the original value for the last element — no extra clone.
                list.push($elem).expect("arrlist![val; n]: capacity overflow");
            }
            list
        }
    };

    // arrlist![a, b, c, ...] — explicit element list
    ($($elem:expr),+ $(,)?) => {
        {
            // Count the number of elements at compile time to pre-allocate exactly.
            let count = $crate::_count!($($elem),+);
            let mut list = $crate::arrlist::ArrayList::with_capacity(count);
            $(list.push($elem).expect("arrlist![...]: capacity overflow");)+
            list
        }
    };
}

/// Helper macro that counts comma-separated tokens at compile time.
///
/// Used internally by [`arrlist!`] to determine the exact capacity needed.
/// Not intended for direct use.
#[doc(hidden)]
#[macro_export]
macro_rules! _count {
    () => { 0usize };
    ($head:expr $(, $tail:expr)*) => { 1usize + $crate::_count!($($tail),*) };
}
