#!/usr/bin/env python3
"""Sentence-level audit for AI-slop patterns in Markdown prose.

Usage: slop_audit.py FILE.md [FILE.md ...]
Skips fenced code, inline math, figures, pseudocode blocks, HTML tags and tables.
Every hit needs a human look: questions inside exercises, `--options` and the
"This chapter covers" heading are expected false positives.
"""
import re
import sys

CHECKS = [
    ("inflated words", r"\b(delve|dive into|deep dive|unpack|unlock|unleash|harness|leverag\w*|empower|elevate|foster|tapestry|landscape|realm|journey|symphony|beacon|testament|crucial|pivotal|vital|robust|seamless|cutting-edge|game.?chang\w*|cornerstone|plays a key role|nuanced|interplay|revolutionar\w*|blazing|incredibly|powerful|elegant)\b", re.I),
    ("hedge or filler", r"(it'?s worth noting|it'?s important to|interestingly|surprisingly|\bsimply\b|\bjust\b|\bclearly\b|\bobviously\b|\bessentially\b|\bbasically\b|of course|in essence|at its core|simply put|deserves? (attention|comment)|merits?\b|as we will see|to see why|nothing is wrong|this is important|both approaches have|it depends on)", re.I),
    ("fake importance", r"(why (it|this) matters|what matters|key (takeaway|insight)|the real story|makes all the difference|earns its place|more than a \w+;|the bottom line|at the end of the day|in summary|tl;dr)", re.I),
    ("dramatic setup", r"(here'?s the thing|let'?s be honest|the truth is|here'?s where|let that sink in|spoiler|the answer\?|the plan follows|there is 1 obstacle)", re.I),
    ("chatty coaching", r"\b(imagine|think of|now ask|look at|let'?s say|pretty much|a little|super|here'?s the nice|don'?t let the)\b|!", re.I),
    ("aggressive verb", r"\b(touch(es|ed)?|hit(s)?|slam(s|med)?|damage[sd]?|kill(s|ed)?|attack(s|ed)?)\b", re.I),
    ("em dash", r"—|\s--\s", 0),
    ("not X but Y", r"\bnot\b[^.,;:]{1,40}\bbut\b|\bnot (just|only|merely)\b|\bthis is no\b", re.I),
    ("rhetorical question", r"\?$", 0),
    ("colon reveal", r"[a-z)]: [a-z]", 0),
    ("personification", r"\b(GPU|CPU|kernel|warp|compiler|hardware|program|thread|block)s? (wants?|feels?|knows?|thinks?|decides?|tries|likes?|cares?|refuses?|insists?|pays?)\b", re.I),
    ("document as actor", r"\b(This|The) (chapter|section|book|part|paper)\b (shows|derives|explains|measures|writes|uses|follows|turns|develops|treats|covers)\b|\b(Chapter|Section|Part) [\dIVX][\d.]* (shows|showed|derives|derived|explains|measured|measures|found|located|turns|removes|develops|developed|writes|uses|used)\b", 0),
    ("missing comma", r"^(In (this|the|our|each|every|Chapter|Section|phase|general|practice|C\+\+|Rust)\b[^,]{0,40}?|For (example|instance)|On (the|a|an)\b[^,]{0,30}?|Finally|However|Therefore|Thus|First|Second|Third|Next|Then|Here) (we|it|they|the|a|an|each|every|this|thread|threads)\b", 0),
    ("mechanical 1", r"\b1 (of (them|these|those)|after another|by 1|at a time|piece|member|example|idea|reason|thread per|single)\b", 0),
    ("vague This/That", r"^(This|That|These|It) (is|was|means|shows|makes|gives)\b", 0),
]


def prose(text):
    text = re.sub(r"```.*?```", "", text, flags=re.S)
    text = re.sub(r"<figure>.*?</figure>|<div class=\"algo\">.*?\n</div>", "", text, flags=re.S)
    text = re.sub(r"<[^>]+>", "", text)
    text = re.sub(r"^\|.*$", "", text, flags=re.M)
    text = re.sub(r"\\\\\(.*?\\\\\)|\\\\\[.*?\\\\\]|\$[^$\n]+\$", "X", text, flags=re.S)
    text = re.sub(r"`[^`\n]*`", "CODE", text)
    text = re.sub(r"\[\^\w+\]", "", text)
    sentences = []
    for para in re.split(r"\n\s*\n", text):
        para = re.sub(r"\s+", " ", para).strip()
        if not para or para.startswith("#"):
            continue
        sentences += [s for s in re.split(r"(?<=[.!?])\s+(?=[A-Z0-9`*(])", para) if s]
    return sentences


def audit(path):
    sentences = prose(open(path, encoding="utf-8").read())
    print(f"### {path}")
    for name, pattern, flags in CHECKS:
        hits = [s for s in sentences if re.search(pattern, s, flags)]
        if hits:
            print(f"== {name}: {len(hits)}")
            for h in hits[:15]:
                print("    " + h[:170])
    heavy_the = [s for s in sentences if len(re.findall(r"\bthe\b", s, re.I)) >= 5]
    if heavy_the:
        print(f"== overused 'the' (5 or more in a sentence): {len(heavy_the)}")
        for h in heavy_the[:10]:
            print("    " + h[:170])
    words = sum(len(s.split()) for s in sentences) or 1
    the_rate = 100 * sum(len(re.findall(r"\bthe\b", s, re.I)) for s in sentences) / words
    long = [s for s in sentences if len(s.split()) > 35]
    print(f"sentences {len(sentences)}, average {words / max(len(sentences), 1):.1f} words, "
          f"'the' per 100 words {the_rate:.1f}, longer than 35 words {len(long)}")
    for s in long[:10]:
        print("    LONG: " + s[:170])


if __name__ == "__main__":
    for p in sys.argv[1:]:
        audit(p)
