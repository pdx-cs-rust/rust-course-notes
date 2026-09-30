# Course Project Proposal

*\[Draft: watch this space and the Zulip for edits.\]*

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
  - The repo must be cloneable by BartMassey on Github. I would
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

This course, the “project theme” is *Instrumented Simulation.*
(This is a repeat of last offering, but it worked out well so I'm
keeping it for one more round.) I’d like you to build some
simulation that evolves over time, together with some kind of
display or logging of what’s happening, and optionally some kind
of simulation controls.

### Examples

- Building a [clicker game / idle
  game](https://en.wikipedia.org/wiki/Incremental_game) is a fun
  project. Games such as [*Universal
  Paperclips*](https://www.decisionproblem.com/paperclips/) or
  *[Cookie
  Clicker](https://cookieclicker.com)* are simulations of
  an imaginary process over time, with minimal user input and
  display. It is not necessary to have a fancy web interface:
  local games with a graphical or ASCII user interface are pretty
  buildable.

  \[This is what inspired the theme.\]

- Biologically-inspired simulations such as
  [*Boids*](https://en.wikipedia.org/wiki/Boids) or [*Ant Colony
  Optimization*](https://en.wikipedia.org/wiki/Ant_colony_optimization_algorithms)
  are interesting. Again, realism and top-notch graphics are not
  necessary: anything works.

- Physically-inspired simulations such as two-dimensional
  [*N-Body
  Simulation*](https://en.wikipedia.org/wiki/N-body_simulation)
  are fun to explore.

- Socially-inspired simulations are fair game. A very simple
  [*Traffic
  Simulation*](https://en.wikipedia.org/wiki/Traffic_simulation)
  would be a challenging but great problem. A toy [*Energy
  Model*](https://en.wikipedia.org/wiki/Energy_modeling) would be
  pretty cool.

- I would prefer staying away from CPU simulators: they are
  typically too much (RISC-V) or too well-trodden (CHIP8) for
  this course. But if your heart is set on it let me know.

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
