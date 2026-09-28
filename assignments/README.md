# Assignments

These Markdown files contain every assignment description from
the course, including assignments that were unpublished. The
descriptive filenames are stable content names.

Descriptions retain the instructor's wording, links, examples,
lists, emphasis, and headings. Formatting inherited from the
learning management system has been removed. Clear errors and
ambiguities may be corrected here because this repository is the
canonical source.

The public files contain teaching material. Course identifiers,
publication state, dates, submission settings, and other
deployment details belong in the ignored `.canvas/` directory.
Keep a separate backup of that directory: Git does not preserve
ignored files.

## Contents

- [Introduce Yourself On Zulip](introduce-yourself-on-zulip.md)
- [Rule 110](rule-110.md)
- [Exercism](exercism.md)
- [Course Project Proposal](course-project-proposal.md)
- [Chomp](chomp.md)
- [Big Bag Of Words](big-bag-of-words.md)
- [Cache](cache.md)
- [sublists_pairs](sublists-pairs.md)
- [Advanced Rust Quiz](advanced-rust-quiz.md)
- [Rust Quiz](rust-quiz.md)
- [Course Project](course-project.md)
- [Course Project Submission](course-project-submission.md)

Matching directories contain available starters, solutions, and
supporting files for these assignments. The Markdown files above
remain the canonical prompts.

## Previous standalone assignments

- [Calculator](calculator/assignment.md)
- [Processing Arguments](processing-arguments/assignment.md)
- [Statistics](statistics/assignment.md)
- [Modular Exponentiation](modular-exponentiation/assignment.md)
- [Toy RSA](toy-rsa/assignment.md)
- [Unique Iterator](unique-iterator/assignment.md)
- [Decision Tree](decision-tree/assignment.md)
- [Keyword Index](keyword-index/assignment.md)

The starter and solution code for Big Bag Of Words is stored in
[its supporting-material directory](big-bag-of-words/).
Keyword Index is its simpler predecessor: it keeps an ordered
list of borrowed words, while Big Bag Of Words adds
case-insensitive normalization, frequency counts, and `Cow`. Both
are retained because their ownership and data-structure exercises
are distinct.

Each imported assignment uses `assignment.md` for its prompt,
`starter/` for student-facing code, `solutions/<variant>/` for
hand-authored solutions, and `assets/` for supporting files. A
directory is omitted when that kind of material was unavailable.
Variant names identify the language or distinguish historical
Rust implementations.

The Chomp materials retain two Rust solutions and historical Java
and Haskell solutions. Modular Exponentiation retains separate
`rust-2022` and `rust-2023` solutions. The Exercism materials
contain the three historical solutions that were available; Hello
World is included there as a prerequisite example.

## Rust solution maintenance

All Rust solution packages use Rust edition 2024. Their
lockfiles are checked in, and shared course code uses local path
dependencies where appropriate. In particular, the Toy RSA
solution uses the library target of the `rust-2023` Modular
Exponentiation solution.

The solutions are maintained with Rustfmt, Clippy with warnings
denied, and their available target and documentation tests.
Registry dependencies require network access on the first build
unless they are already present in the local Cargo cache.

## Quiz assignments

`rust-quiz.md` contains the assignment description for the
existing quiz. The questions, answers, and feedback are stored
in [the quizzes directory](../quizzes/).

`advanced-rust-quiz.md` links to the exported quiz content in
the quizzes directory.

## Project assignments

The proposal, final project, and project submission are separate
assignments with overlapping requirements. They are preserved
separately so that educators can review and choose the material
appropriate for their course.
