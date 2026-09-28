# Rust Programming Course Notes
Bart Massey 2026

These are the ongoing course notes for my Rust Programming
course.

The numbered subdirectories correspond to weeks of my
current course. The `old/` subdirectory contains old notes
in rough order: much of this is duplicate.

The Markdown files in the numbered directories are the
canonical course notes. Canvas HTML is generated from them;
HTML downloaded from Canvas must not be edited as source.

# Building Canvas HTML

Run `make install-pandoc` once to install the pinned Pandoc
release locally, then run `make`. Generated HTML fragments are
written to `build/html/` and checked by parsing them back through
Pandoc.

The file `canvas-notes.tsv` maps each canonical Markdown source
to its unnumbered Canvas filename. Canvas modules follow the same
section boundaries and note order as the numbered directories.

Upload every file in `build/html/` to the Canvas course-files
root with overwrite enabled. Canvas retains the corresponding
module links. Verify the module-item publication state after an
upload; note files are unpublished until they are ready for the
class.

The current notes contain no mathematical notation, so the build
does not load MathJAX. Pandoc will emit MathML if mathematical
notation is added later; the build can be extended with MathJAX
only if Canvas rendering shows that it is needed.

# Assignments And Quizzes

Clean Markdown exports of course assignments and quizzes live in
`assignments/` and `quizzes/`. These public files preserve course
content without Canvas object IDs, publication state, or
scheduling metadata.

Canvas-specific state belongs in the ignored `.canvas/`
directory. That local overlay records the IDs and placement
needed to synchronize this repository with one particular Canvas
course, but it is not canonical course content and must not be
committed.

# License

This work is licensed under the "Creative Commons CC-BY-3.0 Public
License". Please see the file `LICENSE.txt` in this distribution for
license terms.
