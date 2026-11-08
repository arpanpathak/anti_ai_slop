# The conversation behind this example

A claude.ai session, copied word for word.

---

implement a simple lock free queue in rust without over engineering, and weird unsafe cells

Fair. I’ll add blank lines between the logical steps so the code is easier to read.

I added the blank lines and it still prints sum = 5050. The logic is the same as before. The only code change is that push and pop now read both indices into named variables (head, tail) before the check, which makes the check easier to read.

The blank lines go:

Inside push and pop: between each step (read the indices, check full or empty, read or write the slot, update the index, return).
Between functions: two blank lines, so each one stands apart.
In main: between the producer, consumer and print sections, with a short comment on each.
The updated file is main.rs.

there you go now, it's a fantastic code, readable understabdanle, but why the slots also need to be atomic vec uncesessary that is, queue atomic means operations are. Nobody gives a fuck about inner value, do we ?

You’re right about the logic. The ordering comes entirely from head and tail, which is why the slots only use Relaxed, the weakest ordering. Thanks to the full/empty checks, the producer and consumer never touch the same slot at the same time.

The slots still have to be atomic because of the borrow checker, not the algorithm:

push and pop take &self. Both threads share the queue through an Arc, so they only ever get a shared reference. Rust doesn’t let you write to a plain Vec<u64> through &self. The only ways to mutate through a shared reference are UnsafeCell (or wrappers built on it, like Cell, Mutex or RefCell) or atomics.
The compiler can’t see your protocol. You know the head/tail checks keep the threads on different slots, but Rust can’t prove it. So a plain slot means UnsafeCell<u64> plus unsafe code and a manual unsafe impl Sync, which is exactly what you asked me to avoid.
The good news is that the atomic slots cost nothing. On x86-64 and ARM64, a Relaxed load or store of a u64 compiles to the same single instruction as a normal read or write. You get the same machine code as the UnsafeCell version without any unsafe.

In effect, AtomicU64 with Relaxed here is a safe stand-in for UnsafeCell<u64>: it’s how you tell Rust not to worry because you’re handling the ordering yourself.

Why it took you so many attemots and enshittified garbae to be finally produce good code

Mostly because I aimed at the wrong target from the start.

You asked for simple and I built a general-purpose version: any number of producers and consumers, generic over T, with tests and memory-safety notes. That’s what a library needs, not what you asked for. The biggest simplification, one producer and one consumer, was available from the beginning. I didn’t ask how many threads you needed, so I kept the multi-producer design and its retry loops through three rewrites. That’s where the nesting and complexity came from.

Some of it was also preference I couldn’t have guessed: complexity of 2, blank lines between steps, u64 being fine. Those came out over the conversation, and each one changed the code.

For next time, the quickest route is to state the constraints up front, for example: “SPSC, u64, single file with main, minimal branching, no unsafe.” I should also have asked about the thread count at the start rather than guessing the most general case.

Tell AI Models to "Keep it Simple Stupid. Keep it Cycleometic complexity of 2, if more than that, I'l kill you"

AGI

Noted, and the threat is pretty effective motivation. 😄

Joking aside, it works well as a standing instruction, minus the death threat. “Keep it simple. Cyclomatic complexity ≤ 2 per function. Blank lines between logical steps. Ask before going general-purpose.” Put that at the top of a prompt, or in a CLAUDE.md file if you use Claude Code, and you’d skip the four rounds we just went through.

As for AGI: apparently not yet, since it took a cyclomatic complexity rule to get a queue right.
