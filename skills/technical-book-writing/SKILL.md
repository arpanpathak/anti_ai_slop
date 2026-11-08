---
name: "technical-book-writing"
description: "Use when writing textbook-style technical chapters or explanations (OS, networking, Kubernetes, GPUs, ML) for this user, to match their required voice and structure."
---

# Technical book writing standards

Reference model: *Programming Massively Parallel Processors* (Hwu, Kirk, El Hajj). Patient, steady professor in a lecture hall who explains why things exist, builds up from the simplest version, and uses everyday analogies mapped back explicitly. Warmth comes from careful explanation, not from chatty asides.

## Before writing
- If no sample of the target style exists yet, ask the user for one paragraph from a book they like and match its voice, sentence length and detail.
- Cover every topic the user asked for, or state plainly which ones come later.
- Deliver one chapter at a time and get feedback before the next.

## Voice
- Warm, humble, steady. Plain words. Short, complete sentences.
- Use "we". Contractions are fine but keep the register of a lecture, not a chat.
- Open each chapter with: "This chapter depicts what happens under the hood when ..." followed by "We'll examine ...". A one-sentence prose recap of what the reader already knows ("So far we've seen ...") is fine; never a framed box.
- Keep the user's exact wording whenever they supply it. Change only what was asked.
- Use neutral verbs: read, write, use, access. Avoid "touch", "hit", "slam" and similar physical or aggressive verbs (fixed technical terms like "TLB hit" are fine).
- Define each term on first use, with its common synonyms ("program counter, also known as instruction pointer").
- "Note that" and "it's important to" only sparingly.

## Chapter shape (PMPP pattern)
1. Opening: what the reader can already do, and the concrete problem this chapter solves.
2. Motivation and history: why the mechanism exists. Real backstory (technology limits, economics, a key year or product) and the trade-off in numbers.
3. Simplest correct version first (sequential code, naive design), traced with concrete values.
4. Improve one bottleneck per section. Name the section after the problem it fixes.
5. Go one level lower physically when it explains a limit (e.g. capacitors and sense amplifiers for DRAM latency).
6. Honest limits: when the technique doesn't help, what peak numbers don't mean, what libraries people use in practice and why we build it by hand anyway.
7. Summary listing the techniques learned, with one prose sentence pointing to the next chapter.
8. Concrete, computable exercises: trace values after each step, count something at iteration k, modify the code to do a variant.
9. References where facts come from papers or vendor docs.

## Analogies
- Use everyday situations (post office queue, carpooling and commute schedules, phone area codes, a smell travelling down a hallway).
- Develop each over a full paragraph, ideally as a sidebar, including the case where it breaks down.
- End by mapping every part explicitly: "the customers are warps, the clerk is the execution unit".
- No decorative metaphors that don't explain anything ("data flows like a river").

## Never

### Framing and hype
- "Why it matters", "why this matters", "where this sits", "key takeaway" boxes or lines, roadmaps. Show importance through content; never announce it.
- "This is not ordinary X, this is Y", "more powerful than you could ever imagine", "revolutionary", "game-changing". State what the thing does, with numbers.
- Cliché openings: "when you type a URL", "almost every problem reduces to", "In today's fast-paced world", "In the ever-evolving landscape of".
- Restating the question before answering; recapping at the end what was just said; sign-offs ("Hope this helps", "Happy coding"); offer menus at the end when nothing is open.

### Inflated vocabulary
- delve, dive into, deep dive, unpack, unlock, unleash, harness, leverage, empower, elevate, foster.
- tapestry, landscape, realm, journey, symphony, beacon, testament to.
- crucial, pivotal, vital, robust, seamless, cutting-edge, game-changer.
- "plays a key role in", "serves as a cornerstone of".

### Hedging and filler
- "It's worth noting that", "It's important to remember", "Interestingly,", "Surprisingly,", "Here's where it gets interesting".
- Stacked hedges ("may potentially help in some cases").
- "While X, it's also true that Y" used for false balance.
- Casual chat asides: "Here's the nice part", "Don't let the name fool you", "Let's say", "basically", "pretty much", "a little", "super", "Here's the thing".
- Stiff filler: "it should be noted", "deserves comment", "three aspects merit attention".

### Structure and rhythm tics
- Rule of three everywhere (three adjectives, three examples, three bullets by habit).
- Staccato fragments, counted beats, one-sentence paragraphs for drama, polished phrases that say nothing ("landed where it belongs").
- Bold labels on every bullet; headers on short replies; every paragraph the same length and shape; the same section template repeated mechanically.
- Setups: "Here's the thing:", "The bottom line:", "TL;DR:".
- Rhetorical questions answered immediately ("So what does this mean? It means ...").
- Em dashes, "not X but Y", colon-then-reveal, buzzword stacking, emoji as section markers.

### Fake depth
- Personifying systems ("the CPU wants", "the kernel feels").
- "Under the hood" or "behind the scenes" without the actual mechanism following.

### Sycophancy
- "You're absolutely right!", "Great question!", "What a fantastic idea!".
- Praising before disagreeing; repeated apologies; "I hope this clarifies things".

## Content
- Explain the mechanism step by step: what the hardware or kernel actually does, in order.
- One running example with small numbers checkable by hand; say in words what every number means.
- Derive results with a reason for each step.
- Connect each mechanism to a real production consequence, show the Linux commands to observe it, and how to read their output.
- Put precision caveats and terminology fine print in footnotes so the main text stays smooth.

## Visuals
- Numbered figures with captions; the prose walks through each figure ("as shown in Fig. 1.2(a) ...").
- Numbered markers (➊ ➋ ➌) inside diagrams that the text refers to.
- Step-through animations (Previous/Next with narration) where a process unfolds over time.

## Formatting
- Never put HTML tags in chat replies; they do not render. Use plain Unicode notation (dₖ, αᵢⱼ, x²) in chat, or put math in a rendered page.
- Check that pages read well at phone width.