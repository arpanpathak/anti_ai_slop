use std::collections::VecDeque;
use std::mem;

/// Demonstrates std::mem::replace.
/// Swaps the value of a variable with a new value, returning the old value.
/// This is useful for implementing "swap-in" logic without needing a temporary variable.
fn demo_mem_replace() {
    let mut current_value = String::from("old");

    // Replace the current value with a new one.
    // The old value is returned and can be used or discarded.
    let old_value = mem::replace(&mut current_value, String::from("new"));

    assert_eq!(current_value, "new");
    assert_eq!(old_value, "old");

    println!("mem_replace: old={}, new={}", old_value, current_value);
}

/// Demonstrates std::mem::take.
/// A shorthand for mem::replace with the default value of the type.
/// This is particularly useful for clearing vectors, strings, or options.
fn demo_mem_take() {
    let mut buffer = Vec::with_capacity(10);
    buffer.push(1);
    buffer.push(2);

    // Take the current value and replace it with Vec::default() (empty vec).
    let taken_buffer = mem::take(&mut buffer);

    assert!(buffer.is_empty());
    assert_eq!(taken_buffer.len(), 2);

    println!("mem_take: taken_len={}, current_len={}", taken_buffer.len(), buffer.len());
}

/// Demonstrates Option::take.
/// Replaces the Option with None and returns the previous value.
/// Commonly used in iterators or when consuming an option once.
fn demo_option_take() {
    let mut optional_value = Some(42);

    // Take the value out, leaving None in its place.
    let extracted = optional_value.take();

    assert_eq!(extracted, Some(42));
    assert!(optional_value.is_none());

    println!("option_take: extracted={:?}, current={:?}", extracted, optional_value);
}

/// Demonstrates Vec::drain or clearing via replace.
/// Shows how to efficiently clear a collection by replacing it with a new empty one,
/// or using drain if you need to iterate over removed items.
fn demo_vec_drain_and_replace() {
    let mut queue: VecDeque<i32> = VecDeque::from(vec![1, 2, 3, 4, 5]);

    // Drain returns an iterator over the removed items.
    let drained: Vec<i32> = queue.drain(..).collect();

    assert!(queue.is_empty());
    assert_eq!(drained, vec![1, 2, 3, 4, 5]);

    println!("vec_drain: drained_len={}, current_len={}", drained.len(), queue.len());
}

/// Demonstrates std::mem::swap.
/// Swaps the values of two variables. This is atomic and safe.
/// Useful for implementing "ping-pong" buffers or swapping internal state.
fn demo_mem_swap() {
    let mut a = String::from("alpha");
    let mut b = String::from("beta");

    mem::swap(&mut a, &mut b);

    assert_eq!(a, "beta");
    assert_eq!(b, "alpha");

    println!("mem_swap: a={}, b={}", a, b);
}

/// Demonstrates std::mem::forget.
/// Prevents the destructor from running. This is dangerous and should
/// only be used when you explicitly want to leak memory or when
/// the destructor is unnecessary (e.g., raw pointers).
/// In this demo, we leak a Box to show the concept, but in real code,
/// this is rare. We use it here to demonstrate the API safely.
fn demo_mem_forget() {
    let leaked_box = Box::new(42);

    // Forget prevents the destructor from running.
    // The memory is leaked, but the value remains valid in memory.
    mem::forget(leaked_box);

    // We cannot access the value anymore, but it exists.
    // In a real scenario, this might be used with raw pointers or FFI.
    println!("mem_forget: value leaked (not accessible)");
}

/// Demonstrates std::mem::size_of and std::mem::align_of.
/// Useful for understanding memory layout, especially for FFI or custom allocators.
fn demo_mem_size_of() {
    let size = mem::size_of::<u64>();
    let align = mem::align_of::<u64>();

    assert_eq!(size, 8);
    assert_eq!(align, 8);

    println!("mem_size_of: size={}, align={}", size, align);
}

/// Demonstrates std::mem::drop.
/// Explicitly drops a value before the end of its scope.
/// Useful for holding a lock or resource longer than necessary,
/// or for moving a value into a drop guard.
fn demo_mem_drop() {
    let mut data = vec![1, 2, 3];

    // Drop the vector early to free memory before the end of the function.
    drop(data);

    // data is no longer accessible here.
    // We can create a new vector.
    let data = vec![4, 5, 6];

    assert_eq!(data.len(), 3);

    println!("mem_drop: new_data_len={}", data.len());
}

/// Main function to run all demonstrations.
/// Each demo is isolated in its own function to demonstrate idiomatic usage.
fn main() {
    demo_mem_replace();
    demo_mem_take();
    demo_option_take();
    demo_vec_drain_and_replace();
    demo_mem_swap();
    demo_mem_forget();
    demo_mem_size_of();
    demo_mem_drop();

    println!("All demonstrations completed successfully.");
}
