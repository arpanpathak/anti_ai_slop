# Example: a lock-free queue with cyclomatic complexity of at most 2

The request was "implement a simple lock free queue in rust without over engineering, and weird
unsafe cells". It took four rewrites. The first version supported many producers and many
consumers, was generic over `T`, and had retry loops, tests and safety notes. The version the
user called "fantastic code, readable understandable" is `src/main.rs`.

What changed between the first and the last version:

1. **One producer, one consumer, `u64`.** Nobody asked for more. Asking "how many threads?" at
   the start would have skipped three rewrites.
2. **Cyclomatic complexity of at most 2 in every function.** No retry loops inside branches, no
   nesting. Each function has one guard or one loop.
3. **Blank lines between steps.** In `push` and `pop`: read the indices, check full or empty,
   write or read the slot, move the index, return. Two blank lines between functions.
4. **Named values before the check.** `head` and `tail` are read into variables, then compared.
5. **No `unsafe`.** The slots are `AtomicU64` with `Relaxed` ordering. `push` and `pop` take
   `&self`, so a plain `Vec<u64>` cannot be written through it without `UnsafeCell`. A
   `Relaxed` load or store of a `u64` compiles to a single ordinary load or store instruction
   on x86-64 and ARM64, so the safe version costs nothing. The ordering comes from `head`
   (`Acquire`/`Release`) and `tail`.

Run it and measure it:

```sh
cargo run --release              # sum = 5050
lizard src/main.rs               # CCN column: 1 or 2 for every function
```

Measured output of `lizard`:

```
  NLOC    CCN   token  PARAM  length  location
       8      1     61      1       9 new@16-24@src/main.rs
      10      2     97      2      14 push@27-40@src/main.rs
      10      2     89      1      14 pop@43-56@src/main.rs
       3      2     22      2       3 push_spin@59-61@src/main.rs
       7      2     30      1       7 pop_spin@64-70@src/main.rs
      12      1    110      0      22 main@74-95@src/main.rs
```

The full conversation is in `CONVERSATION.md`.

The standing instruction that came out of it:

> Keep it simple. Cyclomatic complexity of at most 2 per function. Blank lines between logical
> steps. Ask before going general-purpose.
