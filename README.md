# arrlist

A generic, heap-allocated dynamic array for `no_std` environments.


[![Crates.io](https://img.shields.io/crates/v/arrlist)](https://crates.io/crates/arrlist)
[![CI](https://github.com/0xMainul/arrlist/actions/workflows/rust.yml/badge.svg)](https://github.com/0xMainul/arrlist/actions)

---

## Under the hood

`ArrayList<T>` uses `Box<[MaybeUninit<T>]>` as its backing store, which means:

- `T` never needs to implement `Default`
- Uninitialized slots are never read as `T` (no UB lurking around)
- Drop glue only runs on elements that were actually written

---

## Features at a glance

- **`no_std` compatible** — needs `alloc`, nothing else
- **Amortized O(1) push/pop** — capacity doubles from 4, like `Vec`
- **O(n) insert, remove, pop_front** — with proper shifting
- **Three iterator flavors** — owned (`IntoIter`), borrowed (`Iter`), and mutable (`IterMut`)
- **Built-in algorithms** — bubble sort, reverse, linear search, binary search
- **Multiple constructors** — from `Vec<T>`, `[T; N]`, `&[T]`, or from scratch
- **Friendly `arrlist![]` macro** — mirrors the `vec![]` syntax you already know

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arrlist = "0.2.7"
```


---

## Quick Start

```rust
use arrlist::{arrlist, ArrayList};

// The macro works just like vec![]
let mut list = arrlist![1, 2, 3];
assert_eq!(list.len(), 3);
assert_eq!(list.get(1), Some(&2));

// Push and pop from the back
list.push(4).unwrap();
assert_eq!(list.pop(), Some(4));

// Pre-allocate with a known capacity
let zeros: ArrayList<i32> = arrlist![0; 8];
assert_eq!(zeros.len(), 8);
```

---

## The `arrlist![]` macro

Three forms, mirroring `vec![]`:

```rust
use arrlist::{arrlist, ArrayList};

// Empty list — no heap allocation yet
let empty: ArrayList<i32> = arrlist![];

// Explicit elements
let nums = arrlist![10, 20, 30];

// N copies of a value (value must implement Clone)
let repeated = arrlist![0_i32; 5];
```

---

## Constructors

```rust
use arrlist::ArrayList;

// Start empty, allocate on first push
let list: ArrayList<i32> = ArrayList::new();

// Reserve space upfront
let list: ArrayList<i32> = ArrayList::with_capacity(16);

// From an array (moves elements, no clone)
let list = ArrayList::from_array([1, 2, 3]);

// From a Vec (reuses the buffer, no copy)
let list = ArrayList::from(vec![4, 5, 6]);

// From a slice (clones each element; T: Clone required)
let list = ArrayList::from_slice(&[7, 8, 9]);
```

---

## Core Operations

```rust
use arrlist::ArrayList;

let mut list = ArrayList::from_array([10, 20, 30]);

// Read
assert_eq!(list.get(0), Some(&10));
assert_eq!(list.get(99), None);

// Write
list.set(0, 100).unwrap();

// Modify in place
if let Some(val) = list.get_mut(1) {
    *val *= 2;
}

// Insert at an arbitrary position (O(n))
list.insert(1, 15).unwrap();

// Remove at an arbitrary position (O(n))
let removed = list.remove(2).unwrap();

// Pop from the back (O(1)) or front (O(n))
list.push(99).unwrap();
list.pop();
list.pop_front();

// Wipe everything (drops elements, keeps allocation)
list.clear();
```

---

## Iterators

All three standard iterator patterns are supported:

```rust
use arrlist::ArrayList;

let list = ArrayList::from_array([1, 2, 3]);

// Shared references
for val in &list {
    println!("{val}");
}

// Mutable references
let mut list = ArrayList::from_array([1, 2, 3]);
for val in &mut list {
    *val *= 10;
}

// Consuming the list
for val in list {
    println!("{val}"); // owns each element
}
```

---

## Algorithms

```rust
use arrlist::ArrayList;

let mut list = ArrayList::from_array([3, 1, 4, 1, 5, 9]);

// Reverse in place — O(n)
list.reverse();

// Bubble sort ascending — O(n²), fine for small or nearly-sorted data
list.sort();

// Linear search — O(n), works on any list
let idx = list.linear_search(&4);

// Binary search — O(log n), list must be sorted first
let idx = list.binary_search(&5);
```

> **Heads up on sort:** The built-in sort uses bubble sort, which is O(n²). It's perfectly reasonable for small lists. If you're dealing with thousands of elements, extract to a `Vec` and use `slice::sort_unstable` instead.

---

## Error Handling

Fallible operations return `Result<_, ListError>`. There are three variants:

| Variant | When you'll see it |
|---|---|
| `ListError::EmptyList` | Index-based op on an empty list |
| `ListError::OutOfBounds { idx, limits }` | Index out of range on a non-empty list |
| `ListError::CapacityOverflow` | Growing would overflow `usize` (practically unreachable on 64-bit) |

```rust
use arrlist::{ArrayList, error::ListError};

let mut list: ArrayList<i32> = ArrayList::new();

match list.remove(0) {
    Err(ListError::EmptyList) => println!("nothing to remove"),
    Ok(val) => println!("removed {val}"),
    _ => {}
}

list.push(1).unwrap();
list.push(2).unwrap();

match list.set(5, 99) {
    Err(ListError::OutOfBounds { idx, limits }) => {
        println!("index {idx} is out of range {limits:?}");
    }
    _ => {}
}
```

---

## Display

`ArrayList<T>` implements `Display` when `T: Debug`, printing in the familiar `[a, b, c]` format:

```rust
use arrlist::ArrayList;

let list = ArrayList::from_array([1, 2, 3]);
println!("{list}"); // [1, 2, 3]
```

---

## no_std Setup

In your crate root, enable the `alloc` extern and make sure your target provides an allocator:

```rust
#![no_std]

extern crate alloc;

use alloc::vec;
use arrlist::ArrayList;
```

---

## Running the Tests

```bash
cargo test
```

The test suite covers construction, element operations, iterators, algorithms, error paths, and macro behavior.

---

## License

MIT — see [LICENSE](LICENSE) for details.
