---
name: keep-it-simple-code
description: Write simple, flat, readable code for this user in any language (Rust, C++, Go, JS, Python). Load it BEFORE writing or reviewing code, and before asking an agent to write code. Covers cyclomatic complexity of at most 2 per function, blank lines between logical groups, descriptive names, no over-engineering, no unwrap or panic, doc comments, and a lizard audit command.
---

# Keep it simple: readable code without slop

Code is read far more often than it is written. The user reads every line, often in a printed
book, and judges code by how easily a person follows it from top to bottom on the first pass.
Code that passes the compiler and clippy can still be rejected as "unreadable functional soup",
"nested if else soup" or "overengineered".

Each rule below records code that the user rejected and what they wanted instead.

## The example this skill comes from

The request was "implement a simple lock free queue in rust without over engineering, and weird
unsafe cells". The first answer supported many producers and consumers, was generic over `T`,
and had retry loops nested in branches, tests and safety notes. It took four rewrites to reach
the code below, which the user called "fantastic code, readable understandable". Every function
has a cyclomatic complexity of 1 or 2 (`lizard` measured), and it prints `sum = 5050`.

```rust
fn push(&self, v: u64) -> bool {
    let tail = self.tail.load(Relaxed);
    let head = self.head.load(Acquire);

    if tail.wrapping_sub(head) == self.arena.len() {
        return false; // full
    }

    self.arena[tail % self.arena.len()].store(v, Relaxed);

    self.tail.store(tail.wrapping_add(1), Release); // publish

    true
}
```

One guard, then one step per group, separated by blank lines. No `unsafe`: the slots are
`AtomicU64` with `Relaxed` ordering, which compiles to an ordinary load or store. The whole
program and the conversation are in `examples/spsc_queue/` of the anti_ai_slop repository.

Aim for this shape in every function you write.

## 1. Ask for the constraints before writing

The most expensive mistake is building the general case when a simple one was asked for. A
request for "a simple lock-free queue" took four rewrites because the first version supported
any number of producers and consumers, was generic over `T`, and had tests and safety notes.
The user needed one producer, one consumer and `u64` values.

- When the request leaves the scope open, ask one short question: how many threads, which
  types, one file or a crate, tests or not.
- Otherwise pick the smallest version that does the job: one file, one `main`, concrete types.
- Do not add generics, traits, builders, caches, modules, helpers or tests that nobody asked for.
- Measure the real data before adding generality. Do not write recursive or multi-level
  handling the data never needs.

## 2. Cyclomatic complexity of at most 2

Every function has at most one decision point: one `if`, one loop or one `match` guard. A
function with two decisions becomes two functions.

- Use guard clauses and early returns. Never nest `if` inside `if`, or `match` inside `if`.
- Use `let ... else`, `continue` and `?` to keep the happy path at the left margin.
- Replace an `else if` chain with a `match` or a lookup table.
- Name a value before using it. Write `let limit = if busy { 4 } else { 8 };` on its own line
  instead of `start(if busy { 4 } else { 8 })`.
- Keep each function short (about 25 lines or fewer) and give it one job, so `main` reads as a
  top-down story.

Check it before showing code:

```sh
pip install lizard
lizard -C 2 -w path/to/src        # prints every function above 2; exit status 1 if any
```

`lizard` reads Rust, C, C++, Go, Java, JavaScript, TypeScript and Python. It counts every
`match` arm after the first and every `?` as a decision, so two warnings are accepted: a
`match` that only dispatches (each arm one call or one value), and a straight line of `?`
calls with no other branch. Everything else above 2 gets split.

## 3. Blank lines between logical groups

The user has rejected code "shoved line after line" in four different repositories. Inside every
function, put a blank line:

- between setup, the work and the result;
- before and after each loop, branch and guard (`if`, `for`, `while`, `match`, `let ... else`);
- before a `return` or a final expression that follows other statements;
- between functions, and between the producer, consumer and printing sections of a `main`.

A comment stays attached to the statement below it. Never write one-line bodies such as
`if (x) { a; b; c; }`. No run of more than about 6 statements without a blank line.

## 4. Names

- Use full, descriptive names: `request_id`, `assistant_reply`, `line_buffer`, `head`, `tail`.
- Never use single letters (`e`, `m`, `x`) or vague names (`acc`, `out`, `rest`, `tmp`,
  `result`, `data`, `mine`, `pick`, `l_rest`), except a loop index in mathematical code where
  the text defines it.
- Use the domain's own names. In CUDA, write `threadIdx.x` and `blockDim.x`, not `tx` and `T`.
- Name functions after what they produce (`question_answer_pairs`), not a mechanical step
  (`split_into_turns`).
- No jargon names such as `Cons` and `Nil`. Use `Node`, `Empty`, `left`, `right`, `current`.
- Use exactly the names the user gives. Do not rename the user's types or aliases.

## 5. Plain control flow over clever code

- Prefer a plain `for` loop with destructuring over a chain of `.map().filter().zip().scan()`
  that needs thought to read. One or two adapters are fine.
- Break a method chain one call per line (`chain_width = 36` to `40` in `rustfmt.toml`).
- Prefer `match` on enums and tuples over chains of boolean helper calls.
- Do not reach for uncommon library methods (`dedup_by`, `split_at_mut`, `Option::insert`,
  `name @ Pattern`) unless asked. Mention them as an aside instead.
- No discarded values: no `let _ =`, `Err(_)`, `|_|` or catch-all `_ =>` when the cases can
  be named.
- No stringly typed state. Use enums (`iota` constants in Go) and typed fields.
- No flag variables driving a state machine when grouping the data first reads better.

## 6. Safety and errors

- No `unwrap()`, `expect()` or `panic!` for ordinary failures, not even in tests. Functions
  return `Result` and use `?`; `main` returns a `Result`.
- Errors are an enum implementing `Display` and `std::error::Error` (by hand or `thiserror`),
  never `String` or `Box<dyn Error>`. `anyhow` is fine in a binary's `main` and in tests.
- Avoid `unsafe` and `UnsafeCell` when a safe type does the job. In the queue example, slots
  of `AtomicU64` with `Relaxed` ordering compile to the same load and store instructions as
  plain memory, with no `unsafe` and no manual `unsafe impl Sync`.
- In Go, return errors with context and handle them once in `main`. No `panic(err)`.

## 7. Documentation

- A `//!` header on every module and a `///` doc comment on every public item, every struct
  field and every non-trivial private function. In JS, JSDoc on functions and option fields.
- Comments say why, not what the next line does. No comments that narrate the code.
- Doc comments follow the anti-slop writing rules too: no "robust", "seamless", "leverage".

## 8. Toolchain and formatting

- Use the latest stable release (`rustup check`), pin it in `rust-toolchain.toml`, and use
  the latest major version of each crate. Do not trust the machine's default toolchain.
- Format with `cargo fmt` or `gofmt`. Keep lines at 92 to 100 characters.
- Compile with no warnings and run clippy before showing code.

## 9. Before showing code

1. Run the code the way the user runs it and paste the real output. Never claim it works
   without running it.
2. Run `lizard -C 2 -w` on the changed files and fix every warning except a pure dispatch.
3. Grep for `unwrap|expect\(|panic|unsafe|let _` and remove each hit.
4. Read every function once looking only for missing blank lines between groups.
5. Reread the code as a reviewer of that language would, and delete anything that exists only
   as plumbing.
6. Change only what was asked. "Fix the code in the PR" means only the lines the PR adds.

## 10. The standing instruction

Put this at the top of a prompt or a `CLAUDE.md`:

> Keep it simple. Cyclomatic complexity of at most 2 per function. Blank lines between logical
> steps. Descriptive names. No unwrap, no unsafe. Ask before going general-purpose.
