# anti_ai_slop

Skills and rules for Claude Code and other agents that keep AI slop out of prose and code.

## Skills

| Skill | Use it for |
|---|---|
| `skills/anti-ai-slop-writing-for-neurodivergent` | Any prose for people: chapters, docs, READMEs, PR and commit text, chat answers. Includes `scripts/slop_audit.py`. |
| `skills/keep-it-simple-code` | Any code: cyclomatic complexity of at most 2, blank lines between logical groups, descriptive names, no over-engineering, no unwrap. |
| `skills/book-figures-pseudocode-formulas` | Figures (Material containers, sharp arrowheads, labels inside boxes), CLRS pseudocode with CUDA names, consistent formula fonts, checking rendered pages. |
| `skills/technical-book-writing` | The chapter shape of a textbook in the style of *Programming Massively Parallel Processors*. |

`examples/spsc_queue/` is the lock-free queue that produced the coding skill: the code the user
approved, its complexity measured with `lizard`, and the conversation word for word.

Install them for Claude Code on this machine:

```sh
./install.sh            # links each skill into ~/.claude/skills
```

`technical-book-writing` already syncs from the claude.ai account, so remove its link after
installing to avoid a duplicate.

Audit a document and a source tree:

```sh
python3 skills/anti-ai-slop-writing-for-neurodivergent/scripts/slop_audit.py chapter.md
pip install lizard && lizard -C 2 -w src/
```

## Anti AI Slopfication with a Harness

![System design of the harness: the task goes to a local model, the answer is saved and checked,
and each failure goes back to the model.](harness/assets/system-design.svg)

![Entity diagram of the harness: struct Harness steps through enum State, and struct Answer
carries one reply from parse to report.](harness/assets/entities.svg)

`harness/` is a command-line tool in Rust, under 200 lines. We give it a task in plain English.
It sends our task to a local model, with `keep-it-simple-code` as its system prompt. Then it
saves the model's program and runs the model's check command. When a check fails, the harness
sends back the compiler error or program output and asks again, for up to 5 attempts. Each
program must assert its own result, so a wrong answer fails its check too.

```sh
cd harness
API_KEY=... cargo run --release -- write a parallel sum algorithm using multiple CPU cores
```

The program lands in `harness_out/`. The model server and model name are the `URL` and `MODEL`
constants at the top of `src/main.rs`. The model chooses the check command, and the harness
runs it as the current user, so run it in a folder where that is safe.

We wrote the loop as a state machine with one `State` enum and one `match`.

```rust
enum State { Ask, Check(String), Fix(Error), Pass, GiveUp }
```

| Measure | Value |
|---|---|
| Lines in `src/main.rs` | 197 |
| Average cyclomatic complexity (`lizard`) | 2.5 |
| Unit tests | 9 |
| Line coverage (`cargo llvm-cov`) | 91.11%: every function except `main` |

`main` reads `API_KEY` and the command line, so unit tests do not run it. The tests of `run`
use a fake model server on localhost. `lizard` counts each `?` as a decision, so `main` and
`send`, which are straight lines of `?`, score 4, and `run`, the dispatch `match`, scores 5.
Every other function scores 1 or 2.

## Sources

`sources/` holds the original guides the skills were built from, unchanged. The skills also
draw on the author's feedback in the GPU book, the GPU Kubernetes book and the online chat in
`examples/spsc_queue/`.

- `anti_ai_slop.md`: the slop taxonomy and the deletion test.
- `WRITING_GUIDE.md`: the writing guide of *Cracking the Systems Programming Interview*.
- `WRITING_STANDARDS.md`: the writing standards of the GPU book.
- `rust-rules/rustfmt.toml`: the formatting settings.

Where a source disagrees with a skill, the skill wins. The skills are newer.

## A note to readers and fellow writers


> I'm tired of seeing AI slop writing. I, therefore, earnestly request everyone to please,
> please, please be considerate about what you're writing.
>
> It creates huge cognitive loads on readers like me with ADHD. Keep it simple, genius. Using an
> LLM is okay for writing, but not thinking about how you can make it better and understandable
> for readers hurts credibility.
>
> Tell your agents not to write these.
>
> 1. The "This is no X. This is Y. Why it matters" pattern. Please avoid hollow sentences that
>    don't depict anything and can't spark curiosity in the reader's mind.
> 2. Aggressive, chatty, rhetorical, and dramatic writing. Instead, write like a warm and helpful
>    professor with patience who teaches every concept with care and in easy-to-understand
>    English.
> 3. Avoid cricket commentary–style writing. "This chapter writes, some more verb, measures and
>    reports the same." Don't use verbs for nonliving things. A chapter can't write by itself.
>    Instead, use "In this chapter, we'll examine and benchmark…"
> 4. Avoid overusing the article "the." "The this, the that." A human writer will creatively
>    vary and not make the reading monotonous.
> 5. Please be concise and thoughtful with your writing. Seek feedback from native English
>    speakers and professors. Clarity, precision, and respecting readers' cognitive load are
>    underrated skills.
>
> Warm regards, fellas
