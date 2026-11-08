---
name: book-figures-pseudocode-formulas
description: Rules for figures, pseudocode and formulas in this user's technical books (mdBook, SVG figures, MathJax). Load it before drawing or changing a figure, writing pseudocode, or writing a formula, and before reviewing a rendered chapter. Covers Material containers, sharp arrowheads, text kept inside boxes, real values in figures, CLRS pseudocode with CUDA names and blank lines between groups, consistent formula fonts, and checking every page in a headless browser.
---

# Figures, pseudocode and formulas

These rules come from the GPU book (`gpu-parallel-book`) and the GPU Kubernetes book. The
author rejected figures with dotted arrow tips, text spilling out of boxes and outlined boxes,
pseudocode with invented one-letter names and no spacing, and formulas set in mixed fonts.
Each rule records one of those rejections.

## 1. Figures

- **Draw the real values a mechanism works on:** the array elements, the queue contents, the
  revisions. Icon cards that show no data were rejected.
- **One idea per figure, each step shown.** A figure comes before the first excerpt of the
  component it explains, and the prose walks through it ("as Figure 14.3 shows, …").
- **Material design containers.** Tonal fills with no outline, rounded corners (8 px or
  more), and a soft two-layer shadow on white cards. Keep outlines only on small cells (under
  20 px tall), on chips and on emphasis rings.
- **Sharp arrowheads with no dot at the tip.** A round line cap past the head looks like a dot.
  Use a swept-back head, `stroke-linecap="butt"` on every line that carries a marker, and
  shorten the line so it ends inside the head while the tip lands on the target. Short arrows
  (under 18 units) get a proportionally smaller head. In the GPU book, `book/tools/fig_arrows.py`
  does this for `line` and `path` elements, including `H` and `V` commands.
- **Every label stays inside its box.** Measure it: load the SVG in a headless browser, call
  `getBBox()` on each text element and the smallest rectangle that contains it, and flag any
  overflow over 1 px. Fix every finding before showing the figure.
- **Names in figures match the text,** including the pseudocode's names and American spelling.
- **Titles, a lede line inside the figure, full alt text and a one-line caption.** Numbered
  markers (➊ ➋ ➌) that the text refers to.
- **Generate figures from a script** (`figkit.py`, `draw_chNN.py`), never by editing the SVG
  by hand. A hand fix is lost on the next render.

## 2. Pseudocode

- **CLRS style.** The procedure name in small capitals, numbered lines, `[a, b)` ranges, and
  the text cites line numbers. When lines change, renumber every reference in the prose.
- **CUDA's own names:** `threadIdx.x`, `blockIdx.x`, `blockDim.x`, `gridDim.x`. Invented names
  (`tx`, `T_x`, `B_x`, `b`, `t`) were rejected.
- **No cryptic one-letter names.** Use `phase`, `offset`, `inner`, `distance`, `slot`,
  `level`, `pixel`, `current`, `next`. Mathematical symbols that a table defines (`x_i`, `μ`)
  stay.
- **A blank line between every group of statements,** the same as code. The author rejected
  pseudocode without it twice.
- **One statement per line.** A line that does not fit is split, or a named value is computed
  on the line before.
- **Bold and readable.** Bold names and keywords, set in a dyslexia-friendly face (Atkinson
  Hyperlegible), with line numbers on the same baseline as their text.
- **No comment in the procedure name.** It renders in small capitals.

## 3. Formulas

- **One font for one name.** A name such as `blockIdx.x` must look the same in running text,
  tables and displayed formulas. Set it as a formula everywhere
  (`\mathsf{blockIdx{.}x}` in MathJax), not in the serif body font in one place and sans in
  another.
- **Upright multi-letter names, explicit products.** Write `phase · T`, never `tT`.
- **Explain every symbol in words, with a value from the running example,** and add a
  `Symbol | Meaning | In the example` table when a formula has more than 2 symbols.
- **Escape for mdBook.** In Markdown, MathJax delimiters are `\\(…\\)`, and a thin space is
  `\\,`. A single `\,` renders as a comma.
- **Notation matches the pseudocode.** If the pseudocode says `row` and `col`, the formula
  does not switch to `r` and `c` without saying so.

## 4. Checking the rendered page

1. Build the book, then open every changed page in a headless browser and take screenshots at
   phone width (390 px) and desktop width (1920 px), in the light and the dark theme.
2. Read each page from top to bottom, and check the formulas, figures and pseudocode as
   closely as the prose. Stale names after a rename are the most common bug.
3. Run the figure overflow check on every figure of the chapter.
4. Do not rebuild a page while a live preview server is rebuilding it.
5. Report what was checked by name ("all 140 figures of Chapters 14 to 24"), and say what was
   not checked.

## 5. Scope

Fix the figure, block or formula that was named. Never delete a figure the author liked while
fixing something near it, and never regenerate every chapter's figures when one was asked for.
