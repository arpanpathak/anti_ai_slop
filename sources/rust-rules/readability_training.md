# Rust Readability Training Set: Functional vs Imperative

<!--
FORMAT
Each example is separated by a line containing only `---`.
Each example has exactly two sections:
  ### Instruction   -> the prompt a user would write
  ### Response      -> the ideal answer
Topic tags are on the line `Tags:` inside the instruction block metadata
comment and can be stripped or kept as you like.

Source: "Functional vs Imperative Rust: Finding the Balance" (author's own guide),
restructured into instruction/response pairs, plus a few extra nuances
(pattern destructuring in for loops, HashMap::entry ownership, review of a
top-k implementation).

Guiding principle for every answer: write the version a reader understands
top to bottom on the first pass. Three tests:
  1. Does it read aloud as a sentence?
  2. Is the data shape fighting you (.0, .1, half-discarded tuple patterns)?
  3. How deep is the indentation?
-->

---

### Instruction

<!-- Tags: iterators, references, iter, iter_mut, into_iter -->
What item types do `iter()`, `iter_mut()` and `into_iter()` give me on a `Vec<i32>`, and how does a `for` loop relate to them?

### Response

Each one yields a different item type:

| Call | Item type | Effect on the vector |
|---|---|---|
| `v.iter()` | `&i32` | shared borrow, vector stays usable |
| `v.iter_mut()` | `&mut i32` | mutable borrow, write with `*x += 1` |
| `v.into_iter()` | `i32` | consumes the vector, it cannot be used afterwards |

A `for` loop calls `IntoIterator::into_iter()` on whatever you hand it, so:

```rust
for x in &v     { }  // same as v.iter()      -> x: &i32
for x in &mut v { }  // same as v.iter_mut()  -> x: &mut i32
for x in v      { }  // same as v.into_iter() -> x: i32, v is moved
```

Maps follow the same rule. `for (k, v) in &counts` gives `(&K, &V)` and leaves the map intact; `for (k, v) in counts` gives owned `(K, V)` and consumes it. When you need a map for the last time, consuming it is often the cleanest option because there is no dereferencing anywhere afterwards.

---

### Instruction

<!-- Tags: patterns, references, for-loop, Copy -->
In this code, `.iter()` already gives references. Why do we write `&num` in the loop instead of just `num`?

```rust
for &num in nums.iter() {
    *counts.entry(num).or_insert(0) += 1;
}
```

### Response

It's *because* `.iter()` gives references that `&num` is there. On the left side of a `for` loop, `&` is not creating a reference. It is a **pattern** that destructures one.

`nums.iter()` yields `&i32`. Matching `&i32` against the pattern `&num` makes the two `&` cancel out, so `num` binds to the inner `i32`. It is the same idea as `let &x = &5;` giving `x: i32`.

Without the pattern:

```rust
for num in nums.iter() {      // num: &i32
    counts.entry(num);        // error: expected i32, found &i32
}
```

The map is `HashMap<i32, usize>`, and `entry` wants the key by value.

Equivalent spellings:

```rust
for &num in nums.iter()        { counts.entry(num); }   // destructure in the pattern
for num in nums.iter()         { counts.entry(*num); }  // dereference manually
for num in nums.iter().copied() { counts.entry(num); }  // iterator yields i32
for &num in nums               { counts.entry(num); }   // nums: &[i32] iterates as &i32 too
```

Caveat: this only works because `i32` is `Copy`. Destructuring copies the value out of the reference. With `String`, `for &s in strings.iter()` fails with "cannot move out of a shared reference", because you would be taking ownership of something you only borrowed.

---

### Instruction

<!-- Tags: HashMap, entry, ownership, get -->
Does `HashMap::entry` expect an owned key? Why, when `get` takes a reference?

### Response

Yes. The signature is:

```rust
pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
```

`entry` might have to **insert** the key if it is missing, and a map owns its keys, so it needs ownership up front. Lookup-only methods (`get`, `get_mut`, `contains_key`, `remove`) take `&Q`, a borrow, because they never store the key:

```rust
counts.get(&num);     // borrow is fine: only looking
counts.entry(num);    // must own: might insert
```

For `i32` this is free, since it's a copy. With `String` keys, `map.entry(word.clone())` clones even when the key already exists and the clone is thrown away. When that matters, check first and only allocate on a miss:

```rust
if let Some(count) = counts.get_mut(word) {
    *count += 1;
} else {
    counts.insert(word.to_string(), 1);
}
```

Often the better fix is to key the map by `&str` borrowed from the input text, so `entry(word)` never allocates at all.

---

### Instruction

<!-- Tags: closures, filter, map, double-reference -->
Why does my `filter` closure get `&&i32` when I iterate with `.iter()`, while `map` gets `&i32`?

### Response

`map` receives each item exactly as the iterator yields it: `&i32` from `.iter()`. `filter` only *inspects* each item before passing it along unchanged, so it borrows the item rather than taking it. Borrowing an `&i32` gives `&&i32`.

Three correct ways to write the same test:

```rust
let a: Vec<_> = numbers.iter().filter(|x| **x > 0).collect();          // dereference twice
let b: Vec<_> = numbers.iter().filter(|&&x| x > 0).collect();          // pattern strips both
let c: Vec<_> = numbers.iter().filter(|x| x.is_positive()).collect();  // method auto-derefs
```

The method version reads best. Method calls dereference automatically through any number of `&`, so when the check exists as a method (`is_positive`, `is_empty`, `starts_with`, `is_alphanumeric`), calling it removes the reference noise entirely.

---

### Instruction

<!-- Tags: auto-deref, deref-coercion, comparisons, arithmetic -->
Where does Rust dereference automatically and where do I need to write `*`?

### Response

1. **Method calls** dereference as many levels as needed, including through `Box`: `x.abs()` works on `&&i32`.
2. **Field access** does too: `boxed_player.age` reaches through the `Box`.
3. **Function arguments** get deref coercion, but only *reference to reference*: `&String` → `&str`, `&Vec<T>` → `&[T]`, `&Box<T>` → `&T`, `&&str` → `&str`. It never turns `&i32` into `i32`.
4. **Arithmetic** is implemented for references to primitive numbers: `x * 2` and `x % 2` work on `&i32`.
5. **Comparisons do not** auto-deref. `x > 0` with `x: &i32` fails because `&i32` and `i32` are different types. Use `*x > 0`, a `|&x|` pattern, or a method.
6. **Building values does not** deref. `heap.push((count, word))` with `count: &usize` stores a reference, not a number.
7. **Returning does not** deref. In `match expr { Expr::Num(value) => *value, ... }` on a `&Expr`, `value` is `&i64` and the `*` is required.

Rule of thumb: before adding a `*`, ask whether you can iterate by value or call a method instead. If neither fits, the `*` is honest and idiomatic. `*counts.entry(word).or_insert(0) += 1` is the standard counter bump, and that star belongs there.

---

### Instruction

<!-- Tags: laziness, iterators, side-effects -->
Why doesn't this code change `total`?

```rust
numbers.iter().map(|x| total += x);
```

### Response

Iterator chains are lazy. A chain is a plan, not an action; nothing runs until something consumes it (`collect`, `sum`, `count`, `max`, `any`, a `for` loop). A bare `map` is never consumed, so the closure never runs. The compiler warns that the `Map` is unused.

Use the right tool for the intent:

```rust
let total: i32 = numbers.iter().sum();     // computing a value
for x in &numbers { total += x; }          // performing a side effect
```

When a chain does run, each item travels through the **whole** chain before the next item starts. For `map` then `filter`, the order is `map 1, filter 10, map 2, filter 20, ...`, not "all maps, then all filters". That fact matters a lot once closures do something with side effects, such as spawning threads.

---

### Instruction

<!-- Tags: collect, type-inference, turbofish -->
How do I control what `collect` builds, and should I use the turbofish?

### Response

Annotate the variable with the container and let `_` infer the element type:

```rust
let zero_nodes: Vec<_>       = (0..n).filter(|&i| in_degree[i] == 0).collect();
let queue: VecDeque<_>       = (0..n).filter(|&i| in_degree[i] == 0).collect();
let letters: BTreeSet<_>     = word.chars().collect();
let reversed: String         = word.chars().rev().collect();
let positions: HashMap<_, _> = word.chars().zip(0..).collect(); // later keys overwrite earlier
```

Prefer `let queue: VecDeque<_> = ...` over `.collect::<VecDeque<_>>()`. The type sits where the reader's eye lands first instead of at the end of a long chain. Use the turbofish when there is no variable to annotate, for example inside an expression: `(start..end).sum::<u64>()`.

---

### Instruction

<!-- Tags: type-alias, signatures, readability -->
How do type aliases make Rust signatures more readable? Show before and after.

### Response

A type alias names a shape so the signature speaks in terms of the problem:

```rust
// Before: the reader must decode what the nested Vec and tuple mean.
fn find_all<T: PartialEq>(grid: &Vec<Vec<T>>, target: &T) -> Vec<(usize, usize)>

// After:
type Grid<T> = Vec<Vec<T>>;   // a rectangle of anything
type Cell = (usize, usize);   // (row, column)

fn find_all<T: PartialEq>(grid: &Grid<T>, target: &T) -> Vec<Cell>
```

Useful aliases for algorithm code:

```rust
type Grid<T> = Vec<Vec<T>>;
type Cell = (usize, usize);
type Graph = Vec<Vec<usize>>;          // graph[node] = neighbours
type Edge = (usize, u64);              // (neighbour, weight)
type WeightedGraph = Vec<Vec<Edge>>;
type Tree = Option<Box<Node>>;
type Interval = [i32; 2];              // [start, end]
type Parsed<T> = Result<T, String>;
```

Two caveats. An alias is only a name: a `Cell` and any other `(usize, usize)` are interchangeable, so the compiler won't stop you mixing a cell with a `(width, height)`. When that mix-up matters, use a newtype like `struct NodeId(usize);`. Also, Clippy prefers `&[Vec<T>]` over `&Vec<Vec<T>>` as a parameter; for interview code the readability of `&Grid<T>` is usually the better trade.

---

### Instruction

<!-- Tags: trait-bounds, generics, PartialOrd, f64 -->
Why does `fn largest<T: PartialOrd>` accept `f64` but `fn largest<T: Ord>` doesn't? How should I pick trait bounds?

### Response

`f64` implements `PartialOrd` but not `Ord`, because `NaN` is not equal to itself, so floats have no total order. A function that only compares items should ask only for `PartialOrd`:

```rust
fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut best = items.first()?;   // empty slice: no largest item
    for item in items {
        if item > best {
            best = item;
        }
    }
    Some(best)
}
```

Asking for `Ord` would lock floats out for no reason.

How to choose bounds: start with none, let the compiler tell you what it needs, and add exactly that. The resulting signature is as permissive as possible and doubles as documentation. Reading `T: PartialOrd` tells you the function only compares; it never prints, clones or mutates, because it didn't ask permission to.

Bounds also follow the data structure you use internally. Counting into a `BTreeMap<T, usize>` needs `T: Ord`; switch to `HashMap` and it becomes `T: Hash + Eq`.

---

### Instruction

<!-- Tags: IntoIterator, generics, API-design -->
Should my generic function take `&[T]` or `impl IntoIterator<Item = T>`?

### Response

If the function just walks the items once, take `impl IntoIterator<Item = T>`. It accepts arrays, vectors, ranges and iterator chains, so the caller never has to `collect` into a vector just to call you:

```rust
fn top_k<T: Ord>(items: impl IntoIterator<Item = T>, k: usize) -> Vec<T> {
    let mut heap = BinaryHeap::new();
    for item in items {
        heap.push(Reverse(item));
        if heap.len() > k {
            heap.pop(); // drop the smallest of the k + 1
        }
    }
    heap.into_sorted_vec()
        .into_iter()
        .map(|Reverse(item)| item)
        .collect()
}

top_k([5, 1, 9, 3, 7], 3);          // array
top_k(words.iter().map(|w| w.len()), 2); // chain, no intermediate Vec
```

Take `&[T]` when you need random access, length, slicing, or multiple passes, or when you return references into the input (`Option<&T>`).

Taking a closure works the same way: `label: impl Fn(&T) -> K` says "give me a way to label each item", and the caller decides the label.

---

### Instruction

<!-- Tags: where-clause, signatures -->
When should I use a `where` clause instead of inline bounds?

### Response

They are identical to the compiler. Use `where` once there are more than two or three bounds, so the function name and parameters stay on one short line and the requirements are listed underneath, one per type:

```rust
fn summarize<T, K>(items: &[T], keys: &[K]) -> String
where
    T: Ord + Display + Clone,
    K: Hash + Eq,
{
    // ...
}
```

---

### Instruction

<!-- Tags: Display, String, allocation, write! -->
What's a cheaper way to join displayable items into a `String` than calling `to_string()` on each one?

### Response

Format directly into the output with `write!` from `std::fmt::Write`, which avoids one temporary `String` per item:

```rust
use std::fmt::{Display, Write};

fn join_with<T: Display>(items: &[T], separator: &str) -> String {
    let mut joined = String::new();
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            joined.push_str(separator);
        }
        write!(joined, "{}", item).unwrap(); // writing to a String cannot fail
    }
    joined
}
```

The bound `T: Display` says "items must be printable, nothing more".

---

### Instruction

<!-- Tags: threads, spawn, laziness, collect -->
Is this a good way to run N worker threads and sum their results?

```rust
let total: u64 = (0..workers)
    .map(|id| thread::spawn(move || work(id)))
    .map(|handle| handle.join().unwrap())
    .sum();
```

### Response

It looks parallel but runs one thread at a time. Iterators are lazy and each item passes through the whole chain before the next starts, so this spawns thread 0, immediately joins it, then spawns thread 1, joins it, and so on.

Collect the handles first so every thread is started before any join:

```rust
let handles: Vec<_> = (0..workers)
    .map(|id| thread::spawn(move || work(id)))
    .collect(); // all threads running now

let total: u64 = handles
    .into_iter()
    .map(|handle| handle.join().unwrap())
    .sum();
```

The details that matter:
- The `collect()` in the middle is not decoration. It forces all spawns before any join.
- `move` gives each thread its own copy of `id`. Without it the closure would borrow `id`, which only lives for one iteration, and a thread may outlive it.
- `handles.into_iter()` is required because `join` takes the handle by value; `iter()` would give `&JoinHandle`.
- `(0..workers)` is already an iterator; no vector of ids is needed.

---

### Instruction

<!-- Tags: scoped-threads, borrowing, Arc -->
How can threads read a vector owned by the caller without `Arc` or cloning?

### Response

Use `std::thread::scope`. The scope guarantees every thread finishes before `scope` returns, so threads can borrow local data directly:

```rust
use std::thread;

let data: Vec<u64> = (1..=100).collect();

let partial_sums: Vec<u64> = thread::scope(|scope| {
    let handles: Vec<_> = data
        .chunks(25) // borrowed slices, nothing copied
        .map(|chunk| scope.spawn(move || chunk.iter().sum::<u64>()))
        .collect(); // start all before joining
    handles.into_iter().map(|h| h.join().unwrap()).collect()
});
```

`move` here moves the slice *reference* `chunk` into the thread, not the numbers. The same laziness rule applies: collect the handles before joining.

---

### Instruction

<!-- Tags: topological-sort, Kahn, indices, VecDeque -->
How should I build the initial queue in Kahn's topological sort? I wrote this:

```rust
let queue: VecDeque<_> = in_degree
    .iter()
    .enumerate()
    .filter(|(_, &degree)| degree == 0)
    .map(|(node, _)| node)
    .collect();
```

### Response

It works, but every closure unpacks a tuple and throws half away. Iterating the vector gives you degrees, while the queue needs node ids. When the result you want is an index, iterate over indices:

```rust
let mut queue: VecDeque<_> = (0..node_count)
    .filter(|&node| in_degree[node] == 0)
    .collect();
```

It reads aloud as the first step of the algorithm: "from nodes 0 to n, keep those with in-degree zero, collect into a queue".

The rest of Kahn's algorithm should stay a `while let` loop, because it mutates several structures while draining a queue that grows as you go. Iterator chains are not built for a collection that changes while you walk it:

```rust
let mut order = Vec::with_capacity(node_count);
while let Some(node) = queue.pop_front() {
    order.push(node);
    for &next in &graph[node] {
        in_degree[next] -= 1;
        if in_degree[next] == 0 {
            queue.push_back(next);
        }
    }
}
// Nodes in a cycle never reach in-degree zero.
(order.len() == node_count).then_some(order)
```

---

### Instruction

<!-- Tags: filter_map, then_some, maps -->
My graph nodes are `char`s and in-degrees live in a `BTreeMap<char, usize>`. How do I build the zero in-degree queue cleanly?

### Response

Each entry is a `(key, value)` pair, so `filter_map` with `then_some` combines the test and the transformation in one closure, with both halves named:

```rust
let queue: VecDeque<_> = in_degree
    .iter()
    .filter_map(|(node, degree)| (*degree == 0).then_some(*node))
    .collect();
```

`true.then_some(x)` is `Some(x)`; `false.then_some(x)` is `None`.

The loop is equally idiomatic and choosing it is not a compromise:

```rust
let mut queue = VecDeque::new();
for (node, degree) in &in_degree {
    if *degree == 0 {
        queue.push_back(*node);
    }
}
```

---

### Instruction

<!-- Tags: any, all, find, position, count, max -->
Rewrite this flag-setting loop idiomatically:

```rust
let mut has_negative = false;
for x in &numbers {
    if *x < 0 {
        has_negative = true;
        break;
    }
}
```

### Response

```rust
let has_negative = numbers.iter().any(|x| x.is_negative());
```

When a loop exists only to set a flag or find one thing, there is usually a method whose name *is* the question:

```rust
numbers.iter().any(|x| x.is_negative())       // is any...?
numbers.iter().all(|x| x % 2 == 0)            // are all...?
numbers.iter().find(|x| x.is_negative())      // which one? -> Option<&i32>
numbers.iter().position(|&x| x > 100)         // where?     -> Option<usize>
numbers.iter().filter(|&&x| x == 0).count()   // how many?
numbers.iter().max()                          // -> Option<&i32>, None if empty
```

`any`, `all`, `find` and `position` stop at the first decisive item, exactly like `break`, so nothing is lost in performance. `max()` returns `Option` because an empty collection has no maximum, which is more honest than starting from `i32::MIN` and hoping the sentinel never leaks into the result.

---

### Instruction

<!-- Tags: windows, underflow, usize, neighbours -->
Why is `for i in 0..v.len() - 1` risky for comparing neighbours, and what should I use?

### Response

`v.len()` is a `usize`. For an empty vector, `0 - 1` underflows: a panic in debug builds, a wrap to a huge number in release builds. Use `windows(2)`, which yields overlapping pairs and simply yields nothing for short input:

```rust
let is_sorted = numbers.windows(2).all(|pair| pair[0] <= pair[1]);
```

Its sibling `chunks(k)` gives non-overlapping pieces, handy for batching. (Recent Rust also has `is_sorted()` directly on slices.)

---

### Instruction

<!-- Tags: strings, palindrome, chars, allocation -->
What's an allocation-free way to check if a string is a palindrome using iterators?

### Response

Compare the character iterator with its own reverse:

```rust
let is_palindrome = text.chars().eq(text.chars().rev());
```

`Iterator::eq` compares item by item, so no reversed `String` is built. For the "ignore case and punctuation" version, clone the iterator; cloning copies only its cursor, not the text:

```rust
fn is_palindrome(text: &str) -> bool {
    let letters = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    letters.clone().eq(letters.rev())
}
```

---

### Instruction

<!-- Tags: sorting, sort_by_key, Reverse, total_cmp, then_with -->
How do I sort structs by an integer field descending, and by a float field with a tie-breaker?

### Response

```rust
use std::cmp::Reverse;

// Integer key, ascending
players.sort_by_key(|p| p.age);

// Integer key, descending
players.sort_by_key(|p| Reverse(p.age));

// Float key (f64 is not Ord): total_cmp. Highest score first, ties by name A..Z.
players.sort_by(|a, b| {
    b.score
        .total_cmp(&a.score)          // b before a: descending
        .then_with(|| a.name.cmp(&b.name))
});

// Largest float
let best = players.iter().map(|p| p.score).max_by(|a, b| a.total_cmp(b));
```

Nuances:
- `f64` has no `Ord` because of `NaN`; `total_cmp` supplies a total order.
- Swapping `a` and `b` inside `sort_by` is the simplest way to sort descending.
- `then_with` only runs when the previous comparison is `Equal`, so tie-breakers chain naturally: "by score, then by name, then by age".
- Avoid `sort_by_key(|p| p.name.clone())`: it allocates a `String` per comparison. Use `sort_by(|a, b| a.name.cmp(&b.name))`.
- `sort_unstable_by_key` is faster and allocation-free but may reorder equal elements. When tie order doesn't matter (plain numbers), it's the better default.

---

### Instruction

<!-- Tags: enumerate, tuples, parallel-arrays, structs -->
This chain finds the name with the highest score among those scoring at least 50. How can I make it more readable?

```rust
let winner = scores
    .iter()
    .enumerate()
    .filter(|(_, &score)| score >= 50)
    .max_by_key(|(_, &score)| score)
    .map(|(index, _)| names[index]);
```

### Response

Every closure unpacks a tuple and discards half of it, and the reader has to track which position means what across four steps. Three better options:

**1. Iterate over indices**, so every closure receives one plain number:

```rust
let winner = (0..names.len())
    .filter(|&i| scores[i] >= 50)
    .max_by_key(|&i| scores[i])
    .map(|i| names[i]);
```

**2. A plain loop** with named variables:

```rust
let mut winner = None;
let mut best_score = 0;
for i in 0..names.len() {
    if scores[i] >= 50 && scores[i] > best_score {
        best_score = scores[i];
        winner = Some(names[i]);
    }
}
```

**3. Fix the data shape** (the real fix). Parallel arrays are what force `enumerate` and `zip`. Keep name and score together:

```rust
struct Player { name: &'static str, score: u32 }

let winner = players
    .iter()
    .filter(|p| p.score >= 50)
    .max_by_key(|p| p.score)
    .map(|p| p.name);
```

Now every closure gets a value with named fields, and the chain reads like English.

---

### Instruction

<!-- Tags: fold, tuple-accumulator, Kadane -->
Is this a good implementation of Kadane's maximum subarray?

```rust
let best = numbers
    .iter()
    .skip(1)
    .fold((numbers[0], numbers[0]), |(best, current), &x| {
        let current = x.max(current + x);
        (best.max(current), current)
    })
    .0;
```

### Response

It's correct but hard to read. A `fold` with a tuple accumulator is a loop with several mutable variables whose names have been erased. The reader has to work out that position 0 is `best` and position 1 is `current`, then find the `.0` at the very end to learn which is the answer. It also panics on an empty slice via `numbers[0]`.

```rust
/// Largest sum of any contiguous run. None for an empty slice.
fn max_subarray(numbers: &[i32]) -> Option<i32> {
    let (&first, rest) = numbers.split_first()?;
    let mut best = first;
    let mut current = first;
    for &x in rest {
        current = x.max(current + x); // extend the run, or start fresh at x
        best = best.max(current);     // remember the best run so far
    }
    Some(best)
}
```

`fold` is lovely with **one** accumulator of obvious meaning. Even then, check whether a named method exists first: `sum`, `product`, `max`, `min`, `count`, `any`, `all`. If the accumulator is a tuple, a loop almost always reads better.

---

### Instruction

<!-- Tags: scan, prefix-sums -->
Should I build prefix sums with `scan`?

```rust
let prefix: Vec<i64> = std::iter::once(0)
    .chain(numbers.iter().scan(0, |sum, &x| {
        *sum += x;
        Some(*sum)
    }))
    .collect();
```

### Response

It's clever, but the loop states the recurrence directly, the same line you'd write on a whiteboard:

```rust
// prefix[i] = sum of numbers[0..i], so len + 1 entries
let mut prefix = vec![0; numbers.len() + 1];
for i in 0..numbers.len() {
    prefix[i + 1] = prefix[i] + numbers[i];
}

// sum of numbers[left..=right] in O(1)
let range_sum = prefix[right + 1] - prefix[left];
```

---

### Instruction

<!-- Tags: for_each, return, break, take_while -->
Why does this add the 5 even though I `return` at the negative number?

```rust
let mut total = 0;
numbers.iter().for_each(|x| {   // [3, 8, -1, 5]
    if x.is_negative() {
        return;
    }
    total += x;
});
// total == 16, expected 11
```

### Response

`return` inside a closure returns from the **closure**, not from the loop, so it behaves like `continue`. `for_each` with a multi-line body is a `for` loop with fewer abilities: no `break`, no `continue`, no `?`, and no `return` from the enclosing function.

Use a real loop:

```rust
let mut total = 0;
for x in &numbers {
    if x.is_negative() {
        break;
    }
    total += x;
}
```

Or an iterator that states the intent, with no side effects hidden in a closure:

```rust
let total: i32 = numbers.iter().take_while(|x| !x.is_negative()).sum();
```

`for_each` is reasonable only at the end of a chain with a one-call body, and even there a `for` loop is just as good.

---

### Instruction

<!-- Tags: and_then, nested-closures, question-mark, grid, checked_add_signed -->
How do I flatten nested `and_then` closures when computing grid neighbours?

```rust
let neighbors: Vec<_> = DIRECTIONS
    .iter()
    .filter_map(|&(d_row, d_col)| {
        row.checked_add_signed(d_row).and_then(|next_row| {
            col.checked_add_signed(d_col).and_then(|next_col| {
                (next_row < rows && next_col < cols).then_some((next_row, next_col))
            })
        })
    })
    .collect();
```

### Response

Use `?` inside the closure. It works in any closure that returns `Option`, turning each check into one flat line. Extract it into a helper that returns a lazy iterator, so callers don't allocate:

```rust
type Cell = (usize, usize);
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn neighbors(cell: Cell, rows: usize, cols: usize) -> impl Iterator<Item = Cell> {
    let (row, col) = cell;
    DIRECTIONS.into_iter().filter_map(move |(d_row, d_col)| {
        let next_row = row.checked_add_signed(d_row)?; // None above the top edge
        let next_col = col.checked_add_signed(d_col)?; // None left of the left edge
        (next_row < rows && next_col < cols).then_some((next_row, next_col))
    })
}
```

- `checked_add_signed` adds a signed step to a `usize` and returns `None` instead of wrapping, so there are no `as i32` / `as usize` casts.
- `move` is needed because the returned iterator carries `row`, `col`, `rows`, `cols` after the function returns.
- Callers loop directly: `for (r, c) in neighbors(cell, rows, cols) { ... }`.

---

### Instruction

<!-- Tags: code-review, chain-length, guideline -->
What's a practical rule for when an iterator chain is too long?

### Response

A guideline that holds up in code review:

- Up to about **four adapters with one-line closures**: keep the chain.
- A closure body longer than one line, a tuple pattern with parts thrown away, or any `.0` / `.1`: pause. Name an intermediate result, move the closure into a named function, or write a loop.
- A **closure inside a closure**: almost always rewrite, often with `?`.

Naming an intermediate result is underrated:

```rust
let passing_players: Vec<_> = players.iter().filter(|p| p.score >= 50).collect();
let best = passing_players.iter().max_by_key(|p| p.score);
```

The name documents what you're holding at that point. The test for any chain: does it read aloud as a sentence?

---

### Instruction

<!-- Tags: if-let, match, Option -->
Should I write `match x { Some(v) => ..., None => {} }`?

### Response

No. A `match` with an empty `None => {}` arm is an `if let` in disguise:

```rust
if let Some(count) = stock.get(fruit) {
    println!("{}: {} in stock", fruit, count);
}
```

Save `match` for when every arm does real work.

---

### Instruction

<!-- Tags: combinators, map_or, unwrap_or, as_deref, parse -->
When are `Option` combinators more readable than `if let`?

### Response

When one or two combinators read left to right without nesting:

```rust
// Transform if present, else default
let length = maybe_name.map_or(0, |name| name.len());

// Look up, try to parse, fall back: "get the port, parse it, otherwise 8080"
let port: u16 = config.get("port").and_then(|t| t.parse().ok()).unwrap_or(8080);

// Borrowed default: a reference either way, nothing copied out of the map
let apples = counts.get("apple").unwrap_or(&0);

// Option<String> -> Option<&str>, which shares a type with a &str default
let label = user.nickname.as_deref().unwrap_or("no nickname");
```

`as_deref()` borrows the string inside, so no new `String` is created just to print a label. Once combinators start nesting (closure inside closure), switch to `?`, `let-else` or `if let`.

---

### Instruction

<!-- Tags: question-mark, if-let-pyramid, as_deref, lifetimes, borrowing -->
Flatten this:

```rust
fn email_domain(users: &Users, id: u32) -> Option<String> {
    if let Some(user) = users.get(&id) {
        if let Some(email) = &user.email {
            if let Some(at) = email.find('@') {
                return Some(email[at + 1..].to_string());
            }
        }
    }
    None
}
```

### Response

```rust
fn email_domain(users: &Users, id: u32) -> Option<&str> {
    let user = users.get(&id)?;           // no such user
    let email = user.email.as_deref()?;   // no email
    let at = email.find('@')?;            // no @
    Some(&email[at + 1..])                // slice of the original, no allocation
}
```

- Each `?` means "if this is `None`, return `None` from the whole function now". What remains is the happy path, top to bottom.
- `as_deref()` turns `&Option<String>` into `Option<&str>`. We only hold a reference to the user, so we can't move the `String` out; `as_deref` (or `as_ref`) borrows instead. You'll use it constantly with struct fields.
- Returning `Option<&str>` instead of `Option<String>`: the domain already exists inside the stored email, so a slice costs nothing. Lifetime elision ties the result to `users`, and the compiler guarantees the map outlives it.

---

### Instruction

<!-- Tags: let-else, guard-clauses, continue, parsing -->
I'm parsing `name,age` lines in a loop and skipping bad ones. How do I avoid nested `if let ... else`?

```rust
for line in input.lines() {
    if let Some((name, age_text)) = line.split_once(',') {
        if let Ok(age) = age_text.parse::<u32>() {
            total_age += age;
        } else {
            println!("skipping {}: bad age", name);
        }
    } else {
        println!("skipping malformed line: {}", line);
    }
}
```

### Response

Use `let ... else`. It works where `?` doesn't (in `main`, in loops where you skip items, in functions returning neither `Option` nor `Result`). The `else` block must leave the scope: `return`, `continue`, `break`, or panic.

```rust
for line in input.lines() {
    let Some((name, age_text)) = line.split_once(',') else {
        println!("skipping malformed line: {}", line);
        continue;
    };
    let Ok(age) = age_text.parse::<u32>() else {
        println!("skipping {}: bad age '{}'", name, age_text);
        continue;
    };

    total_age += age;
    valid += 1;
}
```

Each failure is handled beside the check that detects it, instead of at the bottom where the reader pairs `else`s with `if let`s by counting braces. Past the guards, `name`, `age_text` and `age` are simply there, already unwrapped.

Heuristic: whenever an `if let` has a long body and a short `else` (or just `continue`/`return`), turn it into `let ... else`. It is the most effective single tool against nesting in modern Rust.

---

### Instruction

<!-- Tags: while-let, stack, DFS, unwrap -->
What's wrong with this DFS loop?

```rust
while !stack.is_empty() {
    let node = stack.pop().unwrap();
    // ...
}
```

### Response

Two steps must stay in sync, and every reader has to convince themselves the `unwrap` is safe. `while let` does both in one step and cannot panic:

```rust
fn dfs_order(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut stack = vec![start];
    let mut order = Vec::new();

    while let Some(node) = stack.pop() {
        if visited[node] {
            continue; // reached again by another path
        }
        visited[node] = true;
        order.push(node);

        // reverse so the smallest neighbour is visited first
        for &next in graph[node].iter().rev() {
            if !visited[next] {
                stack.push(next);
            }
        }
    }
    order
}
```

Checking `visited` when a node is **popped** (not only when pushed) means a node may sit on the stack more than once but is processed only once. That is the standard way to write iterative DFS correctly. The `continue` at the top is a guard clause.

---

### Instruction

<!-- Tags: binary-search, Ordering, match, usize-underflow -->
Write binary search in Rust the way an interviewer would like to see it.

### Response

```rust
use std::cmp::Ordering;

fn binary_search(numbers: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = numbers.len(); // half-open range [low, high)

    while low < high {
        let mid = low + (high - low) / 2;
        match numbers[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,  // target is to the right
            Ordering::Greater => high = mid,  // target is to the left
        }
    }
    None
}
```

Points worth saying out loud:
- **match on `cmp`**: three arms for exactly three possibilities, checked by the compiler. No final `else` silently standing for "everything else".
- **Half-open range**: the closed-range version writes `high = mid - 1`. With `usize`, when `mid == 0` that underflows (panic in debug, wrap in release). `[low, high)` with `high = mid` never subtracts.
- **`low + (high - low) / 2`** avoids overflowing `low + high`.
- The standard library: `slice.binary_search(&x)` returns `Result<usize, usize>`, `Ok(position)` or `Err(insertion_point)`. `slice.partition_point(|&x| x < 6)` returns the first index where the predicate turns false, the building block for every "first element at least x" problem.

---

### Instruction

<!-- Tags: enums, match, exhaustiveness, catch-all -->
Should I use a `_ =>` catch-all when matching my own enum?

### Response

Prefer listing every variant of your own enums:

```rust
fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(value) => *value,
        Expr::Add(left, right) => eval(left) + eval(right),
        Expr::Mul(left, right) => eval(left) * eval(right),
        Expr::Neg(inner) => -eval(inner),
    }
}
```

When you later add `Expr::Sub`, every exhaustive `match` stops compiling until you handle it; the compiler walks you to every place that needs updating. A catch-all switches off that safety net: the new variant gets quietly absorbed, and the omission shows up at runtime.

Catch-alls belong on types with too many values to list: integers, `char`, strings.

Binding details: matching on `&Expr` makes every binding a reference (`value: &i64`, `left: &Box<Expr>`). Passing `left` to `eval` works because `&Box<Expr>` coerces to `&Expr` at a function call. Returning `*value` needs the star because returning is not a coercion site.

---

### Instruction

<!-- Tags: match, tuple, truth-table, FizzBuzz -->
How can I flatten nested `if`s that depend on two independent conditions, like FizzBuzz?

### Response

Name both conditions and match on the pair. It becomes a truth table the compiler checks for completeness:

```rust
for i in 1..=15 {
    let by_three = i % 3 == 0;
    let by_five = i % 5 == 0;

    let line = match (by_three, by_five) {
        (true, true) => String::from("FizzBuzz"),
        (true, false) => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        (false, false) => i.to_string(),
    };
    print!("{} ", line);
}
```

All four `bool` combinations are listed, so no catch-all is needed.

---

### Instruction

<!-- Tags: merge, match, slice-get, guards, exhaustiveness -->
Write the merge step of merge sort without three separate `while` loops.

### Response

Match on both "current elements" at once. `slice.get(i)` returns `None` past the end, so the four shapes are exactly the four states of a merge:

```rust
fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    loop {
        match (left.get(i), right.get(j)) {
            (Some(&a), Some(&b)) => {
                if a <= b {
                    result.push(a);
                    i += 1;
                } else {
                    result.push(b);
                    j += 1;
                }
            }
            (Some(&a), None) => { result.push(a); i += 1; } // drain left
            (None, Some(&b)) => { result.push(b); j += 1; } // drain right
            (None, None) => break,
        }
    }
    result
}
```

Why not split the first arm with a guard, `(Some(&a), Some(&b)) if a <= b`? The compiler doesn't reason about guards for exhaustiveness, so it would demand another unguarded `(Some, Some)` arm anyway. Keeping the comparison as an `if` inside one arm leaves all four shapes visible.

---

### Instruction

<!-- Tags: match, ranges, matches-macro -->
How do I match on numeric ranges exhaustively and test a char against several values?

### Response

```rust
fn grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        0..=69 => 'F',
        101.. => panic!("score out of range: {}", score),
    }
}

fn is_vowel(letter: char) -> bool {
    matches!(letter.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
```

The arms together span all of `u32`, so the compiler accepts it with no catch-all: every possible score is accounted for by name. `matches!` turns a pattern into a `bool`, replacing `c == 'a' || c == 'e' || ...` and any `match` whose arms only return `true`/`false`.

---

### Instruction

<!-- Tags: match, guard-clauses, map_err, Result, error-handling -->
Refactor this nested match parser:

```rust
fn parse_setting(line: &str) -> Result<(String, u32), String> {
    match line.split_once('=') {
        Some((key, value)) => match value.trim().parse::<u32>() {
            Ok(number) => {
                if key.trim().is_empty() {
                    Err("empty key".to_string())
                } else {
                    Ok((key.trim().to_string(), number))
                }
            }
            Err(e) => Err(format!("bad number: {}", e)),
        },
        None => Err("missing '='".to_string()),
    }
}
```

### Response

A match inside a match inside an `if`, and the error for the very first check ends up on the last line, far from its cause. There is one happy path with several early exits, so write it as guards:

```rust
type Parsed<T> = Result<T, String>;

fn parse_setting(line: &str) -> Parsed<(&str, u32)> {
    let Some((key, value)) = line.split_once('=') else {
        return Err(String::from("missing '='"));
    };

    let key = key.trim();
    if key.is_empty() {
        return Err(String::from("empty key"));
    }

    let number = value
        .trim()
        .parse::<u32>()
        .map_err(|error| format!("bad number: {}", error))?;

    Ok((key, number))
}
```

- Each error sits beside its cause.
- `map_err` converts the parse error into our `String` error type so `?` can propagate it, the standard way to use `?` when error types differ.
- The key comes back as `&str` borrowed from the input, so a successful parse allocates nothing.

When to keep `match`: when every arm is a real outcome of similar weight (`Less`/`Equal`/`Greater`, enum variants). When one path is the happy one and others are early exits, the match is really a series of guards; write it as guards.

---

### Instruction

<!-- Tags: nesting, readability, principles -->
Why is deep nesting so bad for readability, and what tools flatten it?

### Response

Reading nested code means carrying a mental stack: "inside the loop, the cell is open, the neighbour is in bounds, not yet visited". Most people juggle about four items comfortably; past that they reread from the top, and bugs settle in the `else` branches nobody looks at closely. Flat code has the opposite shape: each guard handles one condition and lets it go, so by the time you reach the real work the stack is empty.

The toolkit:

1. **Guard clauses and early `return`**: handle the edge case first; the rest needs no `else`.
2. **`continue`** at the top of a loop body instead of wrapping everything in an `if`.
3. **`let ... else`** to unwrap or leave.
4. **`?`** in functions and closures returning `Option`/`Result`.
5. **Extract a helper** so a fiddly condition or inner loop gets a name (`neighbors`).
6. **Match on a tuple** instead of two nested `if`s on independent facts.
7. **Iterator methods replace inner search loops**: `any`, `find`, `position`.
8. **Combine conditions**: `if a { if b { .. } }` with no `else` is `if a && b { .. }`.
9. **`max`/`min` instead of conditional assignment**: `best = best.max(x);`.

---

### Instruction

<!-- Tags: BFS, grid, refactor, nesting, VecDeque -->
Refactor this grid BFS shortest path. It's eight levels deep.

```rust
fn shortest_path(grid: &[Vec<u8>]) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dist = vec![vec![usize::MAX; cols]; rows];
    let mut queue = VecDeque::new();
    if grid[0][0] == 0 {
        dist[0][0] = 0;
        queue.push_back((0, 0));
        while !queue.is_empty() {
            let (r, c) = queue.pop_front().unwrap();
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nc >= 0 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if nr < rows && nc < cols {
                        if grid[nr][nc] == 0 {
                            if dist[nr][nc] == usize::MAX {
                                dist[nr][nc] = dist[r][c] + 1;
                                queue.push_back((nr, nc));
                            }
                        }
                    }
                }
            }
        }
    }
    if dist[rows - 1][cols - 1] == usize::MAX { None } else { Some(dist[rows - 1][cols - 1]) }
}
```

### Response

```rust
use std::collections::VecDeque;

type Grid<T> = Vec<Vec<T>>;
type Cell = (usize, usize);
const WALL: u8 = 1;

// `neighbors` is the checked_add_signed helper returning impl Iterator<Item = Cell>.

fn shortest_path(grid: &Grid<u8>) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();
    let target = (rows - 1, cols - 1);

    if grid[0][0] == WALL {
        return None;
    }

    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();
    visited[0][0] = true;
    queue.push_back(((0, 0), 0)); // (cell, steps to reach it)

    while let Some((cell, steps)) = queue.pop_front() {
        if cell == target {
            return Some(steps);
        }
        for (row, col) in neighbors(cell, rows, cols) {
            if grid[row][col] == WALL || visited[row][col] {
                continue;
            }
            visited[row][col] = true;
            queue.push_back(((row, col), steps + 1));
        }
    }
    None
}
```

What changed and which tool did it:
- The start-is-a-wall check became a **guard clause** with early return, removing the outer `if`.
- `while !is_empty()` + `unwrap` became **`while let`**.
- Bounds arithmetic moved into the **`neighbors` helper**: three levels and all casts gone.
- "Is it open?" and "is it new?" became **one combined condition with `continue`**.
- Steps **travel in the queue** with each cell, so the `usize::MAX` sentinel disappears.
- Reaching the target is an **early return**, so the post-processing is gone.
- **`WALL` and `target`** name things the reader would otherwise decode.

Deepest point: four levels, and the deepest block is a single `continue`. Same algorithm, same complexity.

---

### Instruction

<!-- Tags: interview, two-sum, HashMap -->
Write Two Sum in idiomatic Rust.

### Response

```rust
use std::collections::HashMap;

fn two_sum(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new(); // value -> index
    for (index, &number) in numbers.iter().enumerate() {
        let needed = target - number;
        if let Some(&earlier) = seen.get(&needed) {
            return Some((earlier, index));
        }
        seen.insert(number, index);
    }
    None
}
```

- `for (index, &number) in ...enumerate()` unpacks a tuple, but every part gets a meaningful name and nothing is discarded. Unpacking isn't the problem; discarding half of every pattern deep inside a chain is.
- `Some(&earlier)` copies the stored index out of the reference `get` returns.
- Checking for the complement *before* inserting makes one pass work and stops a number pairing with itself. O(n) time and space.

---

### Instruction

<!-- Tags: interview, valid-parentheses, stack, Option-comparison -->
Write Valid Parentheses in Rust.

### Response

```rust
fn is_valid(text: &str) -> bool {
    let mut expected: Vec<char> = Vec::new(); // closers we're waiting for

    for c in text.chars() {
        match c {
            '(' => expected.push(')'),
            '[' => expected.push(']'),
            '{' => expected.push('}'),
            ')' | ']' | '}' => {
                if expected.pop() != Some(c) {
                    return false; // wrong closer, or nothing open
                }
            }
            _ => {} // other characters ignored
        }
    }
    expected.is_empty()
}
```

- Push the **closing** bracket you expect, not the opener you saw. Checking a closer is then one comparison, with no second lookup table.
- `expected.pop() != Some(c)` compares two `Option<char>`s and covers both failures: empty stack (`None`) and wrong bracket (wrong `Some`). No `if let`, no `unwrap`.
- The `_` arm is right here: `char` has over a million values.

---

### Instruction

<!-- Tags: interview, sliding-window, HashMap, max -->
Longest substring without repeating characters, in Rust, with minimal nesting.

### Response

```rust
use std::collections::HashMap;

fn longest_unique_run(text: &str) -> usize {
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut start = 0; // left edge of the window
    let mut best = 0;

    for (position, c) in text.chars().enumerate() {
        if let Some(&previous) = last_seen.get(&c) {
            start = start.max(previous + 1); // never move backwards
        }
        last_seen.insert(c, position);
        best = best.max(position - start + 1);
    }
    best
}
```

Many solutions write `if previous >= start { start = previous + 1; }` inside the `if let`. `start.max(previous + 1)` removes that inner level: if the earlier copy is already outside the window, `max` keeps `start` in place. `"abba"` is the case that breaks solutions without this.

`chars().enumerate()` counts characters, not bytes, which is fine because we never slice. If you need to slice, use `char_indices()` for byte offsets, since a UTF-8 character may span several bytes.

---

### Instruction

<!-- Tags: interview, group-anagrams, entry, or_default, lifetimes -->
Group anagrams without allocating new strings for the words.

### Response

```rust
use std::collections::HashMap;

fn group_anagrams<'a>(words: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut groups: HashMap<Vec<u8>, Vec<&str>> = HashMap::new();
    for &word in words {
        let mut key = word.as_bytes().to_vec();
        key.sort_unstable(); // same for every anagram
        groups.entry(key).or_default().push(word);
    }
    groups.into_values().collect()
}
```

- `entry(key).or_default()` returns `&mut` to the group, creating an empty one if needed: one hash instead of the `contains_key` + `insert` dance.
- Groups hold `&str` slices of the input; the lifetime `'a` says the returned words live as long as the input words.
- For lowercase ASCII, a `[u8; 26]` count array is a faster key (O(k) vs O(k log k) per word), and arrays implement `Hash + Eq`, so it drops straight in.

---

### Instruction

<!-- Tags: code-review, top-k, BinaryHeap, Reverse, complexity -->
Review my Top K Frequent Elements:

```rust
pub fn top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for &num in nums.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }
    let mut heap: BinaryHeap<(usize, i32)> = counts
        .into_iter()
        .map(|(num, count)| (count, num))
        .collect();
    let mut result = Vec::with_capacity(k);
    while result.len() < k {
        match heap.pop() {
            Some((_, num)) => result.push(num),
            None => break,
        }
    }
    result
}
```

### Response

The counting is idiomatic, and consuming `counts` with `into_iter()` gives plain values with no dereferencing. Two improvements:

**1. Complexity.** Heapifying every distinct value is O(n log n) in the worst case. Keeping a min-heap of at most k entries is O(n log k), which is the answer interviewers hope for when k is small:

```rust
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Min-heap of at most k: the weakest of the current best sits on top.
    let mut heap = BinaryHeap::with_capacity(k + 1);
    for (num, count) in counts {
        heap.push(Reverse((count, num)));
        if heap.len() > k {
            heap.pop();
        }
    }

    heap.into_sorted_vec()               // ascending by Reverse = descending by count
        .into_iter()
        .map(|Reverse((_, num))| num)
        .collect()
}
```

`BinaryHeap` is a max-heap with no comparator parameter, so ordering is controlled through the items: tuples compare field by field, and `Reverse` flips the order into a min-heap. `into_sorted_vec` returns ascending order, which for `Reverse` means most frequent first, with no extra sort or reverse. With `k == 0` it correctly returns an empty vector.

**2. The drain loop.** If you keep the full-heap version, the `while` + `match` can be one line:

```rust
std::iter::from_fn(|| heap.pop()).take(k).map(|(_, num)| num).collect()
```

Laziness makes this exact: `take(k)` stops pulling after k items, and `from_fn` stops early if the heap empties.

For an O(n) solution, bucket by frequency (`buckets[count]` is a list of values, counts are at most `nums.len()`) and walk the buckets from high to low.

---

### Instruction

<!-- Tags: BinaryHeap, f64, to_bits, priority -->
How do I put `f64` priorities into a `BinaryHeap`?

### Response

`f64` isn't `Ord`, so it can't go in directly. Options:

- If values are **never negative** (distances, costs, probabilities), `value.to_bits()` gives a `u64` that sorts in the same order; `f64::from_bits` converts back. Wrap in `Reverse` for a min-heap.
- Otherwise, wrap the float in a small newtype that implements `Ord` via `total_cmp`, or use a crate like `ordered-float`.

---

### Instruction

<!-- Tags: interview, islands, DFS, recursion, guard-clause -->
Count islands in a grid of `'1'`/`'0'` characters.

### Response

```rust
const LAND: char = '1';
const WATER: char = '0';

// Turn every land cell connected to `cell` into water.
fn sink(grid: &mut Grid<char>, cell: Cell) {
    let (row, col) = cell;
    if grid[row][col] != LAND {
        return; // water or already sunk
    }
    grid[row][col] = WATER;
    let (rows, cols) = (grid.len(), grid[0].len());
    for next in neighbors(cell, rows, cols) {
        sink(grid, next);
    }
}

fn count_islands(mut grid: Grid<char>) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut islands = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == LAND {
                islands += 1;
                sink(&mut grid, (row, col)); // don't count it again
            }
        }
    }
    islands
}

// Building the grid: two collects working together.
let grid: Grid<char> = ["11000", "11000", "00100", "00011"]
    .iter()
    .map(|line| line.chars().collect()) // inner collect inferred as Vec<char>
    .collect();
```

In recursion the guard clause plays the role `continue` plays in a loop: check the base case, leave, and let the rest assume the interesting case. Mention in an interview that recursion depth grows with island size and could overflow the stack on a huge solid grid; the `VecDeque` BFS is the safe alternative.

---

### Instruction

<!-- Tags: interview, Dijkstra, BinaryHeap, Reverse, stale-entries -->
Implement Dijkstra in Rust with readable code.

### Response

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Edge = (usize, u64);              // (neighbour, weight)
type WeightedGraph = Vec<Vec<Edge>>;
type Distances = Vec<Option<u64>>;     // None = unreachable

fn dijkstra(graph: &WeightedGraph, start: usize) -> Distances {
    let mut distances: Distances = vec![None; graph.len()];
    let mut heap = BinaryHeap::new();
    distances[start] = Some(0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((distance, node))) = heap.pop() {
        // A shorter route was already processed: stale entry.
        if distances[node].is_some_and(|best| distance > best) {
            continue;
        }
        for &(next, weight) in &graph[node] {
            let candidate = distance + weight;
            let improves = match distances[next] {
                Some(current) => candidate < current,
                None => true,
            };
            if improves {
                distances[next] = Some(candidate);
                heap.push(Reverse((candidate, next)));
            }
        }
    }
    distances
}
```

- `while let Some(Reverse((distance, node)))` unpacks the entry fully, by name, in the loop header.
- `Option<u64>` makes "unreachable" a real value instead of a magic `u64::MAX`.
- **Stale entries**: `BinaryHeap` has no decrease-key, so we push a new entry when a shorter route appears. The old one surfaces later and the guard skips it. Without it the answer is still correct, but the algorithm can do far more work.
- `improves` could be `distances[next].map_or(true, |c| candidate < c)`, or `is_none_or` on Rust 1.82+. The explicit `match` is clear on any version.

---

### Instruction

<!-- Tags: interview, union-find, path-compression, mem-swap -->
Write union-find with path compression and union by size.

### Response

```rust
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(count: usize) -> Self {
        UnionFind {
            parent: (0..count).collect(), // everyone is their own root
            size: vec![1; count],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let parent = self.parent[x];
        if parent == x {
            return x;
        }
        let root = self.find(parent);
        self.parent[x] = root; // path compression
        root
    }

    /// Returns false if a and b were already connected.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);
        if root_a == root_b {
            return false;
        }
        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }
        self.parent[root_b] = root_a; // smaller under larger
        self.size[root_a] += self.size[root_b];
        true
    }
}
```

- `(0..count).collect()` is a place where functional style is simply the clearest thing to write.
- `find` names both steps (find the root, then point `x` at it). The one-liner `self.parent[x] = self.find(self.parent[x])` compiles but reads `self.parent[x]` twice and hides the steps.
- `std::mem::swap` makes "hang smaller under larger" one code path instead of two mirrored branches.
- A `false` from `union` means the edge closes a cycle, which is useful for cycle detection and Kruskal's algorithm.

---

### Instruction

<!-- Tags: interview, binary-tree, Option-Box, match, if-let -->
Show idiomatic recursive binary tree functions using `Option<Box<Node>>`.

### Response

```rust
struct Node { value: i32, left: Tree, right: Tree }
type Tree = Option<Box<Node>>;

fn max_depth(tree: &Tree) -> usize {
    match tree {
        None => 0,
        Some(node) => 1 + max_depth(&node.left).max(max_depth(&node.right)),
    }
}

fn sum(tree: &Tree) -> i32 {
    match tree {
        None => 0,
        Some(node) => node.value + sum(&node.left) + sum(&node.right),
    }
}

fn invert(tree: &mut Tree) {
    if let Some(node) = tree {
        std::mem::swap(&mut node.left, &mut node.right);
        invert(&mut node.left);
        invert(&mut node.right);
    }
}

fn in_order(tree: &Tree, out: &mut Vec<i32>) {
    if let Some(node) = tree {
        in_order(&node.left, out);
        out.push(node.value);
        in_order(&node.right, out);
    }
}
```

- The `Tree` alias makes signatures read as you'd say them aloud, instead of `&Option<Box<Node>>` everywhere.
- For recursion, `match` makes the two arms literally the two cases of the definition (empty tree, node with subtrees). `tree.as_ref().map_or(0, |n| ...)` is shorter but tucks the base case into an argument.
- Functions acting only on `Some` use `if let`.
- Matching `&mut Tree` binds `node: &mut Box<Node>`, and `node.left` reaches through the box automatically, so no `*` or `as_mut()` is needed.
- LeetCode uses `Option<Rc<RefCell<TreeNode>>>`, accessed via `borrow()`/`borrow_mut()`; the recursion shape stays the same.

---

### Instruction

<!-- Tags: interview, trie, or_default, question-mark, borrow-checker -->
Implement a trie with insert, contains and starts_with.

### Response

```rust
use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_word: bool,
}

impl Trie {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default(); // walk down, create as needed
        }
        node.is_word = true;
    }

    fn find(&self, prefix: &str) -> Option<&Trie> {
        let mut node = self;
        for c in prefix.chars() {
            node = node.children.get(&c)?; // missing letter: None
        }
        Some(node)
    }

    fn contains(&self, word: &str) -> bool {
        self.find(word).is_some_and(|node| node.is_word)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.find(prefix).is_some()
    }
}
```

- `#[derive(Default)]` is what lets `entry(c).or_default()` create children on demand.
- `?` inside the loop in `find` is the flattest way to follow a path that may not exist.
- `contains` and `starts_with` are one-liners on a single walking helper: do the walk once, ask different questions of the result.
- `node = node.children.entry(c).or_default()` compiles because each step *moves* the mutable reference into the child rather than keeping the parent borrowed. People brace for a borrow-checker fight here and none arrives.

---

### Instruction

<!-- Tags: interview, monotonic-stack, while-let, daily-temperatures -->
Daily Temperatures: for each day, how many days until a warmer one?

### Response

```rust
fn days_until_warmer(temperatures: &[i32]) -> Vec<usize> {
    let mut answer = vec![0; temperatures.len()];
    let mut waiting: Vec<usize> = Vec::new(); // day indices, temperatures decreasing

    for (today, &temperature) in temperatures.iter().enumerate() {
        while let Some(&earlier) = waiting.last() {
            if temperatures[earlier] >= temperature {
                break; // not warmer than that day, nor any beneath it
            }
            answer[earlier] = today - earlier;
            waiting.pop();
        }
        waiting.push(today);
    }
    answer
}
```

`waiting.last()` peeks without removing, and the body decides whether to pop. An alternative puts the whole condition in the header:

```rust
while waiting.last().is_some_and(|&earlier| temperatures[earlier] < temperature) {
    let earlier = waiting.pop().unwrap(); // safe: the condition just saw an item
    answer[earlier] = today - earlier;
}
```

It reads like the algorithm ("while the day on top is colder than today"), at the cost of an `unwrap` justified by the condition. Both are good; be ready to say why you chose one.

The stack stores day indices, not temperatures, because the answer is a distance between days and an index gives you both. Each day is pushed and popped at most once: O(n) despite the nested loop.

---

### Instruction

<!-- Tags: interview, merge-intervals, last_mut, match-guard -->
Merge overlapping intervals in Rust.

### Response

```rust
type Interval = [i32; 2]; // [start, end]

fn merge_intervals(mut intervals: Vec<Interval>) -> Vec<Interval> {
    intervals.sort_unstable_by_key(|interval| interval[0]);

    let mut merged: Vec<Interval> = Vec::new();
    for interval in intervals {
        match merged.last_mut() {
            Some(last) if interval[0] <= last[1] => {
                last[1] = last[1].max(interval[1]); // overlap: stretch it
            }
            _ => merged.push(interval), // empty or a gap: start new
        }
    }
    merged
}
```

- Two arms for the two cases of the algorithm. The guard folds "there is a last interval" and "it overlaps" into one arm instead of `if let` with an inner `if`.
- The catch-all fits because it genuinely covers two situations (empty list, gap).
- `last_mut()` gives `Option<&mut Interval>`, so the edit happens in place. The second arm can still `push` because the mutable borrow isn't used there and the borrow checker knows it has ended.

---

### Instruction

<!-- Tags: interview, two-pointers, saturating_sub, palindrome -->
Valid Palindrome with two pointers, ignoring non-alphanumerics and case.

### Response

```rust
fn is_palindrome_two_pointers(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut left = 0;
    let mut right = bytes.len().saturating_sub(1); // 0 for "", not a panic

    while left < right {
        if !bytes[left].is_ascii_alphanumeric() {
            left += 1;
            continue;
        }
        if !bytes[right].is_ascii_alphanumeric() {
            right -= 1;
            continue;
        }
        if !bytes[left].eq_ignore_ascii_case(&bytes[right]) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
```

This is imperative because it has to be: two positions moving independently, each skipping on its own schedule. It still stays flat with `continue` guards. `saturating_sub(1)` avoids underflow on an empty string, and `right -= 1` inside the loop is safe because `left < right` guarantees `right >= 1`. Saying that invariant out loud is what makes an interview answer convincing.

The iterator version (`letters.clone().eq(letters.rev())`) is the better default when two-pointers isn't specifically asked for.

---

### Instruction

<!-- Tags: principles, functional-vs-imperative, summary -->
When should I choose an iterator chain versus a plain loop in Rust?

### Response

Write the version the reader understands top to bottom on the first pass.

**Choose an iterator chain when** it reads aloud as a sentence and each closure is one line about one value: building a collection from a range, filtering, mapping, asking a question (`any`, `all`, `find`, `position`, `count`, `max`), comparing neighbours (`windows`), sorting by keys.

**Choose a loop when** there are several pieces of running state, when you need `break`/`continue`/`?`/`return`, when the algorithm is a sequence of mutations, or when a collection changes while you walk it (BFS/DFS queues, Kahn's algorithm, monotonic stacks).

**Warning signs in a chain:** tuple patterns that discard half their parts, `.0`/`.1`, multi-line closures, closures inside closures, `fold` with a tuple accumulator, side effects in `for_each` or `map`.

**For `Option`/`Result`:** `if let` for one case, `let ... else` to unwrap-or-leave, `?` when returning `Option`/`Result`, `while let` for queue/stack loops, and `match` for several real outcomes of equal weight.

And watch the indentation. When code drifts right, pause and flatten it. The most important reviewer is you six months from now, slightly tired, with no memory of what `.0` meant.

---

### Instruction

<!-- Tags: quick-reference, cheat-sheet -->
Give me a quick reference: situation → what to reach for in Rust.

### Response

| Situation | Reach for | Why |
|---|---|---|
| Build a collection from a range or collection | `(0..n).map(..).collect()` into `Vec<_>` | Reads as a sentence; element type inferred |
| Keep items whose index you need | iterate over `0..n` | Each closure gets one plain number |
| Any / all / find / where / how many | `any`, `all`, `find`, `position`, `count` | Method name is the question; stops early |
| Compare neighbours | `windows(2)` | No off-by-one, no underflow on empty input |
| Sort by a field | `sort_by_key`, `Reverse`, `then_with` | Declarative; tie-breakers compose |
| Sort or compare floats | `total_cmp` | `f64` is not `Ord` |
| Two or more running values | `for` loop with named variables | A tuple accumulator loses the names |
| Running totals, prefix sums | `for` loop | States the recurrence directly |
| Body needs `break`, `continue`, `?` | `for` loop, not `for_each` | Closures can't control the outer loop |
| Several fallible checks inside a closure | `?` inside the closure | Flat lines instead of nested `and_then` |
| Only the `Some` case matters | `if let` | A one-armed `match` adds nothing |
| Unwrap or leave, in a loop or `main` | `let ... else` | Keeps the happy path flat |
| Unwrap or leave, returning `Option`/`Result` | `?` | Flattest chain of checks |
| Process until stack/queue empty | `while let Some(x) = q.pop()` | No `is_empty` + `unwrap` pair |
| Several real outcomes of equal weight | `match` listing every variant | Compiler checks nothing is missing |
| Two independent conditions | `match` on a tuple | A truth table instead of nested `if`s |
| One happy path, several early exits | guard clauses | Each error beside its cause |
| Conditional max/min | `best = best.max(x)` | Removes an `if` and a level |
| Shape used in many signatures | type alias (`Grid<T>`, `Tree`) | Signatures read in problem terms |
| Generic code | smallest trait bound that compiles | Signature documents behaviour |
| Returning part of an input | borrowed `&str` or slice | No allocation; lifetimes keep it safe |
| Iterating `&[i32]` but need `i32` | `for &x in v` or `.copied()` | Pattern destructures the reference |
| Counting into a `HashMap` | `*map.entry(k).or_insert(0) += 1` | `entry` takes owned key; one hash |
| Top k of n | min-heap of size k with `Reverse` | O(n log k) instead of O(n log n) |
| Deep nesting of any kind | guards, `continue`, helpers | Every level costs reader attention |
