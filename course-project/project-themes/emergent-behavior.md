# Emergent Behavior

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
