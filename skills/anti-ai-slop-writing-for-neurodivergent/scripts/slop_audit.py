#!/usr/bin/env python3
"""Sentence-level audit for AI-slop patterns in Markdown prose.

Usage: slop_audit.py FILE.md [FILE.md ...]

Skips fenced code, inline math, figures, pseudocode blocks, HTML tags and tables.
Every hit needs a human look: questions inside exercises, `--options` and the
"This chapter covers" heading are expected false positives.
"""

import re
import sys
from dataclasses import dataclass


@dataclass(frozen=True)
class Check:
    """One slop pattern and the name it is reported under."""

    name: str
    pattern: re.Pattern[str]


CHECKS: list[Check] = [
    Check(
        "inflated words",
        re.compile(
            r"\b(delve|dive into|deep dive|unpack|unlock|unleash|harness|leverag\w*|empower"
            r"|elevate|foster|tapestry|landscape|realm|journey|symphony|beacon|testament"
            r"|crucial|pivotal|vital|robust|seamless|cutting-edge|game.?chang\w*"
            r"|cornerstone|plays a key role|nuanced|interplay|revolutionar\w*|blazing"
            r"|incredibly|powerful|elegant)\b",
            re.I,
        ),
    ),
    Check(
        "hedge or filler",
        re.compile(
            r"(it'?s worth noting|it'?s important to|interestingly|surprisingly|\bsimply\b"
            r"|\bjust\b|\bclearly\b|\bobviously\b|\bessentially\b|\bbasically\b|of course"
            r"|in essence|at its core|simply put|deserves? (attention|comment)|merits?\b"
            r"|as we will see|to see why|nothing is wrong|this is important"
            r"|both approaches have|it depends on)",
            re.I,
        ),
    ),
    Check(
        "fake importance",
        re.compile(
            r"(why (it|this) matters|what matters|key (takeaway|insight)|the real story"
            r"|makes all the difference|earns its place|more than a \w+;|the bottom line"
            r"|at the end of the day|in summary|tl;dr)",
            re.I,
        ),
    ),
    Check(
        "dramatic setup",
        re.compile(
            r"(here'?s the thing|let'?s be honest|the truth is|here'?s where|let that sink in"
            r"|spoiler|the answer\?|the plan follows|there is 1 obstacle)",
            re.I,
        ),
    ),
    Check(
        "chatty coaching",
        re.compile(
            r"\b(imagine|think of|now ask|look at|let'?s say|pretty much|a little|super"
            r"|here'?s the nice|don'?t let the)\b|!",
            re.I,
        ),
    ),
    Check(
        "aggressive verb",
        re.compile(
            r"\b(touch(es|ed)?|hit(s)?|slam(s|med)?|damage[sd]?|kill(s|ed)?|attack(s|ed)?)\b",
            re.I,
        ),
    ),
    Check("em dash", re.compile(r"—|\s--\s")),
    Check(
        "not X but Y",
        re.compile(r"\bnot\b[^.,;:]{1,40}\bbut\b|\bnot (just|only|merely)\b|\bthis is no\b", re.I),
    ),
    Check("rhetorical question", re.compile(r"\?$")),
    Check("colon reveal", re.compile(r"[a-z)]: [a-z]")),
    Check(
        "personification",
        re.compile(
            r"\b(GPU|CPU|kernel|warp|compiler|hardware|program|thread|block)s? (wants?|feels?"
            r"|knows?|thinks?|decides?|tries|likes?|cares?|refuses?|insists?|pays?)\b",
            re.I,
        ),
    ),
    Check(
        "document as actor",
        re.compile(
            r"\b(This|The) (chapter|section|book|part|paper)\b (shows|derives|explains"
            r"|measures|writes|uses|follows|turns|develops|treats|covers)\b"
            r"|\b(Chapter|Section|Part) [\dIVX][\d.]* (shows|showed|derives|derived|explains"
            r"|measured|measures|found|located|turns|removes|develops|developed|writes|uses"
            r"|used)\b"
        ),
    ),
    Check(
        "missing comma",
        re.compile(
            r"^(In (this|the|our|each|every|Chapter|Section|phase|general|practice|C\+\+"
            r"|Rust)\b[^,]{0,40}?|For (example|instance)|On (the|a|an)\b[^,]{0,30}?|Finally"
            r"|However|Therefore|Thus|First|Second|Third|Next|Then|Here) (we|it|they|the|a"
            r"|an|each|every|this|thread|threads)\b"
        ),
    ),
    Check(
        "mechanical 1",
        re.compile(
            r"\b1 (of (them|these|those)|after another|by 1|at a time|piece|member|example"
            r"|idea|reason|thread per|single)\b"
        ),
    ),
    Check(
        "vague This/That",
        re.compile(r"^(This|That|These|It) (is|was|means|shows|makes|gives)\b"),
    ),
]

NON_PROSE: list[tuple[re.Pattern[str], str]] = [
    (re.compile(r"```.*?```", re.S), ""),
    (re.compile(r"<figure>.*?</figure>|<div class=\"algo\">.*?\n</div>", re.S), ""),
    (re.compile(r"<[^>]+>"), ""),
    (re.compile(r"^\|.*$", re.M), ""),
    (re.compile(r"\\\\\(.*?\\\\\)|\\\\\[.*?\\\\\]|\$[^$\n]+\$", re.S), "X"),
    (re.compile(r"`[^`\n]*`"), "CODE"),
    (re.compile(r"\[\^\w+\]"), ""),
]

PARAGRAPH_BREAK = re.compile(r"\n\s*\n")
WHITESPACE = re.compile(r"\s+")
SENTENCE_BREAK = re.compile(r"(?<=[.!?])\s+(?=[A-Z0-9`*(])")
THE = re.compile(r"\bthe\b", re.I)

HEAVY_THE_TITLE = "overused 'the' (5 or more in a sentence)"
HEAVY_THE_COUNT = 5
LONG_SENTENCE_WORDS = 35
CHECK_EXAMPLES = 15
OTHER_EXAMPLES = 10
SHOWN_CHARACTERS = 170


def strip_non_prose(text: str) -> str:
    """Remove code, math, figures, pseudocode, tags, tables and footnote marks."""
    for pattern, replacement in NON_PROSE:
        text = pattern.sub(replacement, text)

    return text


def is_prose_paragraph(paragraph: str) -> bool:
    """A paragraph is prose when it is not empty and is not a heading."""
    return paragraph != "" and not paragraph.startswith("#")


def paragraph_sentences(paragraph: str) -> list[str]:
    """Split one paragraph into sentences; headings and empty paragraphs give none."""
    flat_paragraph = WHITESPACE.sub(" ", paragraph).strip()

    if not is_prose_paragraph(flat_paragraph):
        return []

    pieces = SENTENCE_BREAK.split(flat_paragraph)

    return list(filter(None, pieces))


def sentences_of(text: str) -> list[str]:
    """Every prose sentence of a Markdown document, in order."""
    paragraphs = PARAGRAPH_BREAK.split(strip_non_prose(text))
    sentences: list[str] = []

    for paragraph in paragraphs:
        sentences.extend(paragraph_sentences(paragraph))

    return sentences


def word_count(sentence: str) -> int:
    """Number of words separated by whitespace."""
    return len(sentence.split())


def the_count(sentence: str) -> int:
    """Number of times "the" appears, in any case."""
    return len(THE.findall(sentence))


def is_heavy_the(sentence: str) -> bool:
    """True when "the" appears 5 or more times."""
    return the_count(sentence) >= HEAVY_THE_COUNT


def is_long(sentence: str) -> bool:
    """True when the sentence has more than 35 words."""
    return word_count(sentence) > LONG_SENTENCE_WORDS


def print_examples(sentences: list[str], limit: int, prefix: str) -> None:
    """Print up to `limit` sentences, each cut to 170 characters."""
    for sentence in sentences[:limit]:
        print(prefix + sentence[:SHOWN_CHARACTERS])


def print_hits(title: str, hits: list[str], limit: int) -> None:
    """Print a heading with the number of hits, then examples; nothing when there are none."""
    if not hits:
        return

    print(f"== {title}: {len(hits)}")
    print_examples(hits, limit, "    ")


def print_summary(sentences: list[str]) -> None:
    """Print sentence count, average length, "the" rate and the long sentences."""
    sentence_total = max(len(sentences), 1)
    word_total = max(sum(map(word_count, sentences)), 1)
    the_rate = 100 * sum(map(the_count, sentences)) / word_total
    long_sentences = list(filter(is_long, sentences))

    print(
        f"sentences {len(sentences)}, average {word_total / sentence_total:.1f} words, "
        f"'the' per 100 words {the_rate:.1f}, longer than 35 words {len(long_sentences)}"
    )
    print_examples(long_sentences, OTHER_EXAMPLES, "    LONG: ")


def audit(path: str) -> None:
    """Print every check's hits and the summary for one Markdown file."""
    with open(path, encoding="utf-8") as markdown_file:
        sentences = sentences_of(markdown_file.read())

    print(f"### {path}")

    for check in CHECKS:
        hits = list(filter(check.pattern.search, sentences))
        print_hits(check.name, hits, CHECK_EXAMPLES)

    heavy_the_sentences = list(filter(is_heavy_the, sentences))
    print_hits(HEAVY_THE_TITLE, heavy_the_sentences, OTHER_EXAMPLES)

    print_summary(sentences)


def main() -> None:
    """Audit every file named on the command line."""
    for path in sys.argv[1:]:
        audit(path)


if __name__ == "__main__":
    main()
