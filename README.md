# anti_ai_slop

Skills and rules for Claude Code and other agents that keep AI slop out of prose and code.

## Skills

| Skill | Use it for |
|---|---|
| `skills/anti-ai-slop-writing-for-neurodivergent` | Any prose for people: chapters, docs, READMEs, PR and commit text, chat answers. Includes `scripts/slop_audit.py`. |
| `skills/keep-it-simple-code` | Any code: cyclomatic complexity of at most 2, blank lines between logical groups, descriptive names, no over-engineering, no unwrap. |
| `skills/technical-book-writing` | The chapter shape of a textbook in the style of *Programming Massively Parallel Processors*. |

Install them for Claude Code on this machine:

```sh
./install.sh            # links each skill into ~/.claude/skills
```

Audit a document and a source tree:

```sh
python3 skills/anti-ai-slop-writing-for-neurodivergent/scripts/slop_audit.py chapter.md
pip install lizard && lizard -C 2 -w src/
```

## Sources

`sources/` holds the original guides the skills were built from, unchanged:

- `anti_ai_slop.md`: the slop taxonomy and the deletion test.
- `WRITING_GUIDE.md`: the writing guide of *Cracking the Systems Programming Interview*.
- `WRITING_STANDARDS.md`: the writing standards of the GPU book.
- `rust-rules/five_rules.md`: the five Rust rules used to score models.
- `rust-rules/readability_training.md`: instruction and response pairs on functional and
  imperative Rust.
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
