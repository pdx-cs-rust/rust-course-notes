# Course Project Proposal

Please write a *PDF* proposal of not more than three pages
describing what you want to do for your project this quarter.
Include:

- Your name and email address.

- A good descriptive project name. This name will be used as the
  name of your repo and for grading. Things like “Rust Class
  Project” or your name or whatever are not acceptable.

- The project topic area.

- The specific project vision. Be detailed.

- Discussion of any issues of concern you might have.

- A git repo URL. I strongly prefer Github: if you really want to
  use something else let me know and we’ll figure it out.

  - The repo should contain a brief README describing the
    project.
  - The repo must be cloneable by `BartMassey` on Github. I would
    strongly prefer that you just make it public. If you do this,
    please add an appropriate open source license: I recommend
    MIT + Apache 2.0 for Rust projects. If you really want to
    keep it private, let me know and we’ll make sure I can get to
    it.

## Submission

Place your PDF writeup in a file called `proposal.pdf` at the top
of your repo. Then submit the repo URL.

## Course Project Parameters

Course projects are due during Finals Week.

Project requirements are as follows:

- One or more Rust crates. The project can be a binary crate, a
  library crate or some combination. A library crate plus a small
  driver application is a really nice project style; something to
  consider.

- The project should involve something vaguely like 500-1500
  lines of code (size doesn’t matter so much as effort and
  quality) performing a coherent function.

- The project should include a `README.md` writeup describing
  what was built, how it worked, what didn’t work, and what
  lessons were learned.

- Code should be written in a reasonable modular style. Code must
  not have `rustfmt` or `clippy` errors. Code should include
  internal unit tests where needed to check correctness. Items in
  crate public interfaces must have good `rustdoc`. The project
  commit history must accurately reflect project development.

- The project must build using stable Rust, unless otherwise
  approved. `unsafe` code should be kept to an absolute minimum
  and clearly documented for both safety and need.

- *Not allowed:* Projects that are strongly derivative of code
  already on the Internet; for example, a “snake” game or a
  workout tracker. Projects must consist mostly of original code,
  and must have minimal resemblance to code already out in the
  world. The usual plagiarism rules apply.

- “Double-dipping” — using the same project for this course and
  another current course — is allowed and encouraged, if *and
  only if* you get the other instructor’s explicit permission.
  You may not re-use past course projects. Obviously you may not
  re-use other people’s projects.

Projects are individual unless you get explicit approval from me:
I will give this approval only in cases where a team really makes
sense and I am convinced that all members will contribute. There
will be extra documentation requirements for team projects.

## Project Theme

This course, the “project theme” is *Emergent Behavior.* I'd
like you to build some software that involves interesting
large-scale behavior arising from interactions among simple
rules.

Emergent behavior happens when simple things interacting
with each other produce interesting or unexpected
results. No single part of the system needs to know what the
whole system is doing.  The interesting behavior comes from
the interactions among the parts.

You shouldn't try to discover some new scientific phenomenon
or build anything complicated. The idea is to write a Rust
program with some simple rules and see what interesting
behavior they can produce.

## Examples

- Biologically-inspired simulations such as
  [*Boids*](https://ccl.northwestern.edu/netlogo/models/Flocking)
  are a classic example. Each bird follows a few simple
  rules about how to move relative to nearby birds, but
  together they form complex, moving flocks. No bird is in
  charge. Realistic physics and fancy graphics are not
  necessary or even particularly desirable.

- Generative art is another possibility. For example,
  [*Diffusion-Limited Aggregation*](https://thecodingtrain.com/challenges/34-diffusion-limited-aggregation)
  produces elaborate branching patterns by letting randomly
  moving particles stick to a growing shape. A few simple
  rules can produce surprisingly beautiful results.

- Simple games can have emergent behavior too.
  [*Chain Reaction*](https://chain.kaiross.in/) is a board game
  where players place pieces on a grid. When a square gets too
  full, its pieces spread to neighboring squares, sometimes
  triggering a whole chain of reactions. Very simple rules
  can produce quite complicated changes to the board.

- [*Cellular Automata*](https://en.wikipedia.org/wiki/Cellular_automaton)
  are a particularly nice source of emergent behavior.
  [*Conway's Game of Life*](https://playgameoflife.com/) has
  just a few rules for whether a cell lives or dies, but
  produces moving objects, repeating patterns, and even
  structures capable of computation. There are lots of other
  cellular automata worth exploring, too.

These are just examples: simulations, games, art, and other
kinds of software are all fair game. I'm not looking for
scientific realism, sophisticated graphics, or an ambitious
project. A simple program with interesting behavior is
plenty. The focus of the course is learning Rust, so pick
something manageable and have fun with it.

## Theme Opt-Out

If you are *passionate* about doing something that does not fit
the project scheme, please do propose what you want to do: say “I
would like to opt out of the Project Theme” somewhere near the
beginning of your proposal. Approval of opt-outs is not
automatic: the instructor will review such proposals especially
carefully, and may choose to ask for a different proposal.

The Project Theme is pretty broad: hopefully you’ll be able to
color within the lines somewhere.

## Tutorials and AI

It’s fine to look at tutorials for ideas and inspiration. Your
project should not closely follow a tutorial, though: do
something original.

In this project I prefer that you not *write* Rust with AI: the
point of the project is to learn to write Rust by hand. That
said, AI is excellent at working on ideas, finding algorithms,
etc.

As always, any tutorials or AI used must be thoroughly documented
in the project writeup.

## Discussion

Please use the course Zulip chat channel `#project` for general
project discussion.

## Project Due Date

The course project itself will be due during Finals Week. We will
set up a separate assignment for that.

## AI Acknowledgement

ChatGPT did the writeup of the Emergent Behavior theme under
the direction of Bart Massey.
