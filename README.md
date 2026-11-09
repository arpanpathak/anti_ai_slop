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

`harness/` is a command-line tool in Rust, about 330 lines. We give it a task in plain English,
and we send that task to a local model with `keep-it-simple-code` as the system prompt. We save
the model's program, run the model's check command in a sandbox, and send each compiler error or
program output back to the model, for up to 5 attempts. Each program must assert its own
result, so a wrong answer fails its check too.

```sh
cd harness
API_KEY=... cargo run --release -- write a parallel sum algorithm using multiple CPU cores
```

The program lands in `harness_out/`. The harness also saves each request and raw reply there;
`harness/README.md` shows a full run and how to read those files. The model server and model name
are the `URL` and `MODEL` constants at the top of `src/main.rs`.

We treat the check command as untrusted, because the model writes it. We run each check with
`bwrap` in new Linux namespaces, and we mount only `/usr`, the Rust toolchain and the program,
all read-only. The working directory is an empty tmpfs, and we discard it when the check exits.
Inside the sandbox, the network namespace has only a loopback interface, and the environment
holds only `PATH`, `HOME` and `RUSTUP_HOME`. We cap CPU time, memory and file size with
`prlimit`, and we stop a check after 120 seconds with `timeout`. We reject a `file_name` that
contains a directory, such as `../../.bashrc`. On a host where we cannot create the sandbox, we
stop before the first request to the model. The limits of the sandbox are listed in
`harness/README.md`.

We wrote the loop as a state machine with one `State` enum and one `match`.

```rust
enum State { Ask, Check(String), Fix(Error), Pass, GiveUp }
```

| Measure | Value |
|---|---|
| Lines in `src/main.rs` and `src/sandbox.rs` | 228 and 101 |
| Average cyclomatic complexity (`lizard`) | 2.5 |
| Unit tests | 14 |
| Line coverage (`cargo llvm-cov`) | 93.43%: every function except `main` |

Our unit tests do not call `main`, because it reads `API_KEY` and the command line. We test
`run` against a fake model server on localhost, and in 5 tests we attempt to escape the
sandbox. We measure complexity with `lizard`, which counts each `?` as a decision. With that
count, `main` and `send`, which are straight lines of `?`, score 5 and 6, and `run`, the
dispatch `match`, scores 5. Every other function scores 1 or 2.

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
