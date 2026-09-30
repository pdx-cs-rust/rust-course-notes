# Rust Programming Course Notes
Bart Massey 2026

These are the ongoing course notes for my Rust Programming
course.

The numbered subdirectories correspond to weeks of my
current course.

The Markdown files in the numbered directories are the
canonical course notes. Canvas HTML is generated from them;
HTML downloaded from Canvas must not be edited as source.

# Building Canvas HTML

Run `make install-pandoc` once to install the pinned Pandoc
release locally, then run `make`. Complete standalone documents
are written to `build/html/`. Canvas Page fragments are written
separately to `build/canvas-pages/`. Both forms are checked by
parsing them back through Pandoc and validating their UTF-8.

The file `canvas-notes.tsv` maps each canonical Markdown source
to its unnumbered generated output name. Canvas modules follow
the same section boundaries and note order as the numbered
directories. Course-specific Page slugs and IDs belong in the
ignored `.canvas/` overlay.

Canvas course notes should be created as Pages from the fragments
in `build/canvas-pages/`. Pages render in Canvas without a file
preview frame. Canvas Pages have no folder hierarchy; the course
modules provide their organization. The standalone documents in
`build/html/` remain useful for other publishing targets.

The current notes contain no mathematical notation. Both output
forms nevertheless ask Pandoc to emit native MathML when notation
is added later. Current browsers and Canvas Pages render that
MathML without requiring MathJAX.

# Assignments And Quizzes

Clean Markdown exports of course assignments, quizzes, and project
materials live in `assignments/`, `quizzes/`, and
`course-project/`. These public files preserve course content
without Canvas object IDs, publication state, or scheduling
metadata.

Canvas-specific state belongs in the ignored `.canvas/`
directory. That local overlay records the IDs and placement
needed to synchronize this repository with one particular Canvas
course, but it is not canonical course content and must not be
committed.

# License

This work is licensed under the "Creative Commons CC-BY-3.0 Public
License". Please see the file `LICENSE.txt` in this distribution for
license terms.
