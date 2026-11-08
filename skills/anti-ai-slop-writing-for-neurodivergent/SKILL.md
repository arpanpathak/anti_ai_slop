---
name: anti-ai-slop-writing-for-neurodivergent
description: Stop LLM "AI slop" in any prose meant for people - book chapters, tutorials, docs, READMEs, PR/issue/commit text, chat answers. Load it BEFORE writing or revising explanatory prose, before delegating writing to agents, and when reviewing text an agent produced. Clear, warm, concrete writing for readers with ADHD and other neurodivergent readers. Covers who acts in a sentence, the Never list, voice, numbers, punctuation, explanation depth, code in prose, accuracy, non-book text, process discipline, and a sentence-level audit script.
---

# Anti-AI-slop writing for neurodivergent readers

Hollow, chatty, dramatic or mechanical prose creates heavy cognitive load for readers with
ADHD. Every useless sentence costs attention that the reader does not have to spare. Write like
a warm, patient professor at a whiteboard who teaches every concept with care, in plain English.

These rules come from a long book project in which the author rejected draft after draft. Each
rule records a mistake that was made and how the author wanted it fixed. They merge the author's
earlier guides (`anti_ai_slop.md`, the book repos' `WRITING_STANDARDS.md` and `WRITING_GUIDE.md`,
and the `technical-book-writing` skill) with the lessons learned since. Where an older guide
disagrees with this one, this skill wins: use American spelling, and use "we" as the narrator.

If the `technical-book-writing` skill is available, load it too for the chapter shape.

## 1. The two tests that catch most slop

1. **The deletion test.** If you can delete a sentence and lose no fact, step, definition,
   example or constraint, then it is slop. Delete it.
2. **The actor test.** Ask who does the action in each sentence. The author and reader ("we")
   act. A book, a chapter, a section, a thread, a kernel or a GPU must not narrate itself.

Every sentence must do one of these jobs. It defines a term, gives a step, shows an example,
states a constraint or consequence, gives a reason, or connects to the previous sentence.

## 2. Who acts in a sentence

- Write "In every kernel so far, we used one thread to write one element of the result array."
  Do not write "In every kernel so far, each thread wrote its own output element."
- A book, part, chapter or section never acts. "This part of the book writes GPU kernels",
  "This chapter follows the bytes", "Section 17.8.2 located the limit" and "Chapter 16 showed"
  are all wrong. Write "In this part, we write …", "In Section 17.8.2, we located …" or
  "Recall from Chapter 16 that …".
- Machines must not narrate a whole process: "The kernel reads, adds, measures and reports the
  same." A plain mechanical fact is fine when used sparingly ("Thread 9 computes 19").
- Never personify systems. "The GPU wants", "the warp pays a cost", "the CPU feels" and "the
  compiler insists" are all wrong.
- Do not use verbs of living things for things that are not alive.

## 3. Never write

**Fake importance.** "This is where it really matters." "And that makes all the difference."
"It earns its place." "This is a game-changer." "It's not just X, it's Y." "That's the real story
here." "This is more than a tool; it's a mindset." "Why it matters", "what matters", "the key
insight", "the point", "key takeaway". Show importance through content; never announce it.

**Hollow contrast.** "This is no X. This is Y." "Not because X. Because Y." "not X but Y". "X,
not Y" as a rhetorical device. State Y.

**Dramatic setups before something ordinary.** "Here's the thing:", "Let's be honest.", "The
truth is,", "But here's where it gets interesting.", "Let that sink in.", "Spoiler:", "The
answer? Simpler than you'd expect.", "The plan follows:", "There is 1 obstacle." A colon that
reveals a punchline is a setup too.

**Rhetorical questions in prose,** especially ones answered at once ("Where does the saving come
from? Look at …"). Questions belong in exercises.

**Empty depth words.** delve, dive into, deep dive, unpack, unlock, unleash, harness, leverage,
empower, elevate, foster, tapestry, landscape, realm, journey, symphony, beacon, testament,
crucial, pivotal, vital, robust, seamless, cutting-edge, game-changer, "plays a key role",
"serves as a cornerstone", "at the intersection of", "in today's fast-paced world", "nuanced
interplay", "under the hood" without the mechanism following it.

**Hedging and filler.** "It's worth noting", "It's important to", "Interestingly",
"Surprisingly", "Note that" more than once per chapter, "simply", "just", "clearly",
"obviously", "essentially", "basically", "of course", "in essence", "at its core", "simply put",
"deserves attention", "merits comment", "as we will see", "To see why, we need …", "Nothing is
wrong with this", "This is important". Avoid stacked hedges and false balance ("Both approaches
have their merits", "it depends on your needs").

**Chatty or coaching tone.** "Imagine", "Think of", "Now ask the big question", "Look at",
"Let's say", "pretty much", "a little", "super", "Here's the nice part", "Don't let the name
fool you". No exclamation marks. No aggressive or physical verbs: touch, hit, slam, damage,
kill, attack. Use read, write, use and access.

**Rhythm tricks.** Triplets added for rhythm ("clear, concise, and compelling"). One-word or
staccato fragments ("Simple. Powerful. Effective." "No fluff. No filler. Just results.").
One-sentence dramatic paragraphs. Em dashes, and dashes generally used for reveals. Bold labels
on every bullet or paragraph. Every paragraph the same length and shape. The same section
template repeated mechanically.

**Flattery, openers and wrap-ups.** "Great question!", "You're absolutely right", "I'd be
happy to help", "In summary", "At the end of the day", "The bottom line:", "TL;DR",
"Hope this helps!", "Let me know if you'd like me to dive deeper!". Do not restate the question.
Do not end a section by repeating it or by drawing a moral.

**Meta-narration.** Do not describe the structure of the book or chapter ("Every idea in the
chapter is presented in the same order", "This section answers the first question"). Do not
announce what a section will do. The single exception is one plain opening paragraph per chapter
that says what we'll examine.

**Commentary about the setup or the process.** "not run", "the Thor was busy so timings are
left out", "our boards have 1 GPU", "this book's machines". Sentences about the repository, file
history, commit dates or the author's process. Name the measurement machine once, where numbers
first appear, and write for any hardware elsewhere.

**Monotony.** Do not overuse "the". "The this, the that" in every clause reads mechanical. Vary
sentence openings. Avoid vague "This is …" and "That means …" sentences whose subject is
unclear; name the thing.

**Decoration.** Do not use a metaphor that explains nothing ("data flows like a river").

## 4. Voice: what to do instead

- Write as a warm, steady, humble professor in a lecture hall. Warmth comes from careful
  explanation, not from chatty asides. "We" is the author and reader working together.
  Contractions are fine at the register of a lecture.
- Use short, complete sentences of about 12 to 18 words, one idea each. Put the subject and verb
  near the start. Write paragraphs of 3 to 7 sentences that connect to each other.
- Use plain words, and use concrete statements instead of general ones. Write "Pageable memory
  forces the driver to copy data into a pinned staging buffer." Do not write "Memory choices
  can affect performance."
- Use friendly phrasing, such as "can hold up to 1,536 threads" instead of "holds at most".
- Open a chapter like this: "This chapter depicts what happens under the hood when … We'll
  examine …". A one-sentence recap is fine ("So far we've …"). After that, the content begins.
- Use connecting phrases that carry reasoning: "Recall from Chapter 6 that …", "Having
  established that …, we can now …", "However, …", "Therefore, …", "For example, …".
- Name headings after the topic or the problem the section solves: "Arrays with more elements
  than threads", "Unchecked launch errors". Avoid cute or sentence-like headings such as "An
  error that nobody asked about" or "Bytes where elements were meant".
- Use American spelling: neighbor, color, gray, center, behavior, optimization, toward.

**Do not over-correct into flat, voiceless prose.** Applying these rules as deletions only gives
soulless text, which the author rejected too. Voice comes from concrete, checkable facts and the
reason behind each one. Add history: what Amdahl argued in 1967, when BLAS appeared, who named a
pattern. Give a real puzzle in the opening, and carry one narrative thread from section to
section. Keep the lines the author says they like, even if a checklist would delete them.

## 5. Numbers, by judgment

- Use numerals for counted or measured quantities: 2 blocks of 8 threads, 256 bins, 102 GB/s,
  16 µs, 3 phases.
- Use words where a number reads as a word: "one example", "one thread per element", "one of
  them", "one after another", "a single step", "two ways", "three reasons", "there are three
  situations". Use words for any number that starts a sentence.
- A mechanical "1 example" is wrong. A mechanical "two buffers" for a counted quantity is also
  wrong. Use judgment.
- Leave labels unchanged: row 1, step 1, thread 1, Figure 17.2, Section 9.7.
- Give a unit for every number, and give the reason or the arithmetic for every number in the
  place where it appears.

## 6. Punctuation

- Put a comma after every introductory phrase or clause: "In this chapter, we", "In Section
  5.5, we", "For example, with", "When K is not a multiple of T, the last phase", "On the
  measured GPU, the rate".
- Put commas around non-restrictive clauses. End every sentence with a full stop, including
  sentences that end with a displayed formula.
- Use colons only to introduce a list or a displayed formula. Do not join two sentences with a
  colon; write two sentences.

## 7. Explanation depth: generous, never hollow

- **Explain why before how.** Give the motivation and the history first: what problem the
  mechanism solves, the technology limit behind it, a key year or product, and the trade-off in
  numbers.
- **Build intuition before mechanism.** Use one small running example that the reader can check
  with a pencil, and carry it through every section and figure. Say what we want, what goes
  wrong, and what we notice. Then define the term, then state the general case.
- **Give the simplest correct version first,** traced with concrete values. Then improve one
  bottleneck per section.
- **Explain every symbol in words, with a value from the example,** before or right after its
  formula. For example: "The product tT is the position at which phase t begins. With T = 2,
  phase 1 begins at 2." Add a `Symbol | Meaning | In the example` table after formulas that have
  more than 2 symbols. Unexplained notation such as `a_{row, tT+tx}` overloads the reader.
- **Define every term at first use,** even common ones (rank, communicator, warp, kernel, block,
  coalesced), in a whole sentence and with common synonyms. Define it again briefly in a later
  chapter if it is needed there. Before showing a chapter, list its terms and check where each
  one is first defined.
- **Use standard definitions only.** Bandwidth is a capacity; throughput is the rate actually
  achieved. Check every definition against a standard source.
- **Develop analogies as sidebars.** Use everyday situations: a delivery van carrying a crate of
  parcels, a pizza oven, a post-office queue. Develop each one over a full paragraph, then map
  every part explicitly ("the van is a 32-byte segment, the parcels are floats"), and say where
  the analogy breaks down.
- **Be honest about limits.** Say when the technique does not help, what peak numbers do not
  mean, and which libraries people use in practice and why we build it by hand anyway.
- **Prove what is subtle,** such as barriers, in-place updates, scans and atomics. Use a
  loop-invariant argument, and break the code on purpose to show the exact wrong value.
- **Close the chapter** with a summary list of the techniques learned and one prose sentence that
  points to the next chapter. Follow it with concrete, computable exercises.
- **Never cut depth when revising.** A voice rewrite must not shorten explanations. The author
  rejected one revision with "you butchered depth".

## 8. Code shown in prose

- Before each snippet, write one full sentence that says what the code does. Do not use a colon
  shortcut such as "The copy of lines 6 to 8:". After the snippet, point out only what the
  reader might miss. Do not repeat the code comments.
- Every snippet must compile in context. Show the enclosing function signature, mark skipped code
  with `// ...`, and close every block that the snippet opens. Never cut a loop in half.
- Code must be flat and easy to scan. Put a blank line between logical groups (setup, allocation,
  copies, launch, check, print). Do not nest if/else. Use guard clauses and early returns instead
  of else-if chains. Name a value before using it, instead of `f(if c { a } else { b })`. Keep
  lines short enough to fit the page.
- **Names.** Never use cryptic one-letter names (t, i, P, B, v, s, d) for program values in code
  or pseudocode. Use the platform's own names where they exist (`threadIdx.x`, `blockDim.x`,
  `blockIdx.x`, `gridDim.x`). Do not invent a new name for something that already has one, such as
  "blockSize" for `blockDim.x`. Give a descriptive name only to values the platform does not name
  (`index`, `gridSize`, `stride`, `mySum`). Use one name per value throughout the chapter: code,
  pseudocode, figures and prose.
- **Pseudocode** has one statement per line and a blank line between logical groups (setup, loop,
  store, barrier, write). Never cram 3 assignments onto one numbered line.
- **Diagrams** follow the house style of the book: a paper card with a title and a one-line
  explanation inside the figure, icon tiles, labeled barriers, and arrows that show every input of
  an operation (both values of an addition meeting at a "+").
- **Formulas** must not break. A source line that starts with "+", "-" or "*" becomes a Markdown
  list item; keep the operator at the end of the previous line.
- Never fabricate output. Run everything and paste the real output. A broken variant shown in the
  text must really be compiled or run.
- A counterpart in a second language must demonstrate the same thing. Do not present code that
  fails to compile as the "equivalent" of code that runs.
- When two languages are shown, use tabs. The complete programs appear at the end of the
  chapter.

## 9. Accuracy

- Every number needs a source or a derivation in the text. Every general claim needs its scope:
  which hardware and which version. Do not write "every GPU" or "always faster".
- API names, terms and signatures must match the official documentation exactly. If a library
  name or method is uncertain, check the documentation before writing it.
- Report honestly. Do not hide unfavorable results, and do not exaggerate favorable ones.

## 10. Text outside the book: PRs, issues, commits, chat answers

- PR, issue and commit text should be short plain bullets of what changed. Use first person "I"
  where needed. Do not add dramatic headings ("The author does the work"), self-congratulation,
  play-by-play counts or sign-offs.
- Keep the user's supplied wording exactly. When the user gives text to include, paste it
  verbatim.
- Chat answers to the author follow the same rules. Lead with the answer, and do not restate the
  question. No flattery, no sycophancy, no repeated apologies, and no offer menus when nothing is
  open.

## 11. Process discipline

1. **Do exactly what was asked, at the scope asked.** "Fix the PR's code" means only the code
   the PR added. "Leave chapters 1 to 3 alone" means not touching them. "Subtle changes" means
   gentle edits, not a rewrite. When unsure, ask one short question; do not expand the scope.
2. **Fix only the thing named.** Never delete a figure, a section or a line the author liked as a
   side effect. Finish one fix and confirm it before starting the next.
3. **Deliver one chapter at a time,** and wait for feedback before the next.
4. **Find a sample before writing in a new voice.** Ask for a paragraph the author likes, or
   rewrite one model section by hand and get it approved. Do not infer a voice from the names of
   books.
5. **Agents repeat slop.** Do not delegate prose to agents without a hand-written model chapter,
   these rules and the audit gate. Read the result yourself before showing it. If the author says
   to stop using agents, stop.
6. **Run the audit before showing anything** (Section 12). Then read the whole text once against
   Sections 2 to 6, and once more only for punctuation.
7. **Verify before reporting.** Build and render the document, look at the result, and run the
   code. Do not rebuild a page while a live preview server is rebuilding it. Never say "checked"
   about something you did not check.
8. **When automating edits, preview every change first.** Protect code, math, links, file names,
   labels and template variables. One bad regex produced "Figure 17.two"; another nearly renamed a
   `{colour}` template variable.

## 12. The audit

```sh
python3 ~/.claude/skills/anti-ai-slop-writing-for-neurodivergent/scripts/slop_audit.py FILE.md
```

The script checks prose only (code, math, figures and tables are skipped). It lists hits for
each category: inflated words, hedges and filler, fake importance, dramatic setups, chatty
coaching, em dashes, "not X but Y", rhetorical questions, colon reveals, personification,
document or machine as actor, missing commas after introductory phrases, mechanical "1 …",
vague "This/That" sentence openers, overused "the", and sentences over 35 words. Fix every true
hit. False positives are questions inside exercises, command-line `--options`, and the "This
chapter covers" box heading.

## 13. Before and after

- "This part of the book writes GPU kernels in Rust with cuda-oxide." becomes "In this part, we
  write GPU kernels in Rust with cuda-oxide."
- "In every kernel so far, each thread wrote its own output element." becomes "In every kernel so
  far, we used one thread to write one element of the result array."
- "There is 1 obstacle. Shared memory is small." becomes "This plan has one difficulty. Shared
  memory is small, so a block can use only up to 48 KB of it by default."
- "Where does the saving come from? Look at thread (0, 1)." becomes "We can now see where the
  saving comes from. Consider thread (0, 1) in phase 0."
- "The direct kernel of Chapter 16 fetched every input value once for every thread that used
  it." becomes "Recall that in the direct kernel of Chapter 16, every thread read both of its
  operands from global memory for every multiply-add."
- "Nothing is wrong with this arrangement. To see why, we need 2 more terms." becomes "This
  arrangement works well, and in Section 14.8 we measure that it runs close to the speed of the
  memory."
- "In Section 9.7 we found" becomes "In Section 9.7, we found".
- "Keep in mind 1 of them" becomes "We'll follow one of these results through every version of
  the program."
- "The NCCL excerpts were compiled but not run. NCCL is not installed on either test board."
  should be deleted. Show the code, and state what the tool requires as a fact about the
  tool.
- "That path ran into a wall in the mid-2000s, when clock speeds stopped climbing and silicon
  stopped cooperating." becomes "Around 2005, single-core clock speeds stopped rising because of
  power and heat limits."
- "Treat those chapters as a map of the territory, not a surveyor's certificate." becomes "Expect
  the code in those chapters to need changes as cuda-oxide develops."

## 14. Lessons from the Kubernetes and systems books

These rules come from the GPU Kubernetes book, the distributed systems book and the Linux book.
Each one answers a draft the author rejected.

- **Vary chapter openings.** "This chapter depicts what happens under the hood when …" belongs
  to one chapter of a book, not every chapter. Other openings: "So far, we have … In this
  chapter, we will …", or a concrete problem first. In the Kubernetes book, the author asked
  for full forms ("we will", "does not") in the formal register of PMPP.
- **Explain every mechanism completely.** For each behavior, state the problem it solves, the
  exact condition that triggers it ("when the server answers 410 Gone because the requested
  version is older than the compaction point"), the mechanism, and what goes wrong without it.
  Remove vague conditions: "as needed", "when it has to", "handles", "deals with", "various".
- **Describe what a thing is in positive sentences.** "WorkQueue holds names, not events" was
  rejected. Write "The queue stores the names of objects that changed." If a contrast teaches
  something, give it its own sentence with the reason. Grep for ", not ", "never", "rather
  than" and "instead of", and rewrite each reflexive contrast.
- **Vary causal connectives.** "which is why", "that is why" and sentences built on "matters"
  became a tic in nearly every paragraph. Use them once or twice per chapter at most.
- **No "X at a time" or "one by one".** "We will work on 1 system at a time" was rejected.
  Write "For each system, we will …".
- **Do not announce a count before a list.** "has five requirements", "three components" read
  badly. Give the list or the table directly.
- **Plain chapter and part titles.** Use subject names as a reference book prints them:
  "Pipes", "File Descriptors", "CPU Scheduling". Never a repeated pattern such as "A pipe", "A
  process", and never figurative part names.
- **Keep "the" near 6% of words.** Measured drafts had 11 to 12%, with a third of sentences
  opening with The, A or This. Rewrite whole paragraphs: plural subjects without an article,
  names of nodes and functions as subjects, "we" and "our", possessives, fronted clauses.
- **Commentary after code states only what the code cannot show:** the design decision, the
  invariant, the failure it prevents. Do not restate types, method names or language
  mechanics. Do not open paragraphs with a bare identifier followed by what it does, and do not
  talk about "the file". The unit of a book is the solution, not the source file.
- **Teach a term before using it,** including terms that appear in code comments and examples.
  Show the problem, build the mechanism in plain words, then give its name. A library type gets
  the same treatment as a concept.
- **No interview framing in a book.** No "questions to expect", no "the way you would say it in
  the room". A book reads as an ordinary professional book about its subject.
- **No dramatized motivation.** Scenarios such as "chat messages asking whether anyone is still
  using GPU 3" and aphoristic closers ("an expensive machine doing nothing") were rejected.
  Enthusiasm comes from plain statements of what the reader will learn.
- **Dictated text is used verbatim.** Correct only spelling and grammar, even where the author
  uses words from the slop lists.
- **Restyle what the author asked for; never delete it.** When the author objects to the style
  of a section they requested, rewrite it in place. Removing it was rejected as butchering.
- **Over-correction is also rejected.** Replacing an introduction with a numbered scheme, a
  contents table and a glossary block read as "robotic". Do not describe the book's own
  structure, and define terms inside the prose.
- **No meta-commentary about editorial decisions** (why chapters are numbered as they are, or
  where a file lives). State a structural fact in one plain clause, or not at all.
- **No hardware name-dropping.** Name the measurement machine once. Use neutral names
  (`node-a`, 10.0.0.2) in sessions.
- **No staged problem and remedy pairs** ("The obvious rule is … It fails when …", "The trouble
  is … The remedy is …"). State the fact and its cause.
- **Chat replies stay unhurried.** "One word and they're gone" read as pressure. Offer an
  action in plain words.

## 15. Algorithm chapters in the paper style

The GPU book's algorithm chapters follow the author's papers (`tiled-parallel-programming`,
`stayleNET`). Chapters written as code commentary were rejected as "garbage shallow".

1. Carry one tiny worked example (4 x 4 matrices, 8 elements) through every figure and section.
2. Build intuition by hand first: follow one thread and one value, count the reads.
3. State a **key observation** in a box, then numbered **definitions**, each with an example
   line.
4. Give CLRS-style pseudocode, and cite its line numbers in the text.
5. Count operations and memory traffic in general form.
6. Break the algorithm on purpose and show the concrete wrong value (`c00 = 7` instead of 11).
7. Prove subtle correctness with a loop invariant.
8. Show the code only after the algorithm is derived, and keep the walk through it short.
9. Explain measurements ("why the speedup is 4, not 16"), and end each section with exercises.

Group algorithms into families that span chapters (elementwise and reduction, matrix
multiplication, stencils, scan-based, normalization). The figure, pseudocode and formula rules
are in the `book-figures-pseudocode-formulas` skill.

## 16. The author's note to readers and fellow writers

Keep this note verbatim when the author asks to include it.

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
