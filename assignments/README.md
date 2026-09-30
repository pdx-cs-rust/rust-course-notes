# Assignments

These Markdown files contain every assignment description from
the course, including assignments that were unpublished. The
descriptive directory and standalone filenames are stable content
names.

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
- [Rule 110](rule-110/assignment.md)
- [Exercism](exercism/assignment.md)
- [Chomp](chomp/assignment.md)
- [Big Bag Of Words](big-bag-of-words/assignment.md)
- [Cache](cache/assignment.md)
- [sublists_pairs](sublists-pairs/assignment.md)

Assignment directories contain the canonical prompt as
`assignment.md`, together with any available starters, solutions,
and supporting files.

## Previous assignments

- [Calculator](calculator/assignment.md)
- [Processing Arguments](processing-arguments/assignment.md)
- [Statistics](statistics/assignment.md)
- [Modular Exponentiation](modular-exponentiation/assignment.md)
- [Toy RSA](toy-rsa/assignment.md)
- [Unique Iterator](unique-iterator/assignment.md)
- [Decision Tree](decision-tree/assignment.md)
- [Keyword Index](keyword-index/assignment.md)

The prompt, starter, and solution code for Big Bag Of Words are
stored in [its assignment directory](big-bag-of-words/).
Keyword Index is its simpler predecessor: it keeps an ordered
list of borrowed words, while Big Bag Of Words adds
case-insensitive normalization, frequency counts, and `Cow`. Both
are retained because their ownership and data-structure exercises
are distinct.

Each directory-backed assignment uses `assignment.md` for its prompt,
`starter/` for student-facing code, `solutions/<variant>/` for
hand-authored solutions, and `assets/` for supporting files. A
directory is omitted when that kind of material was unavailable.
Variant names identify the language or distinguish historical
Rust implementations.

The Chomp materials retain a reference Rust solution, the historical
`rust-coltharp` Rust solution, and historical Java and Haskell
solutions. Modular Exponentiation retains separate `rust-2022` and
`rust-2023` solutions. The Exercism materials contain the three
historical solutions that were available; Hello World is included
there as a prerequisite example.

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

## Quizzes

Quiz instructions, questions, answers, and feedback are stored in
[the quizzes directory](../quizzes/). Canvas represents a quiz as
an assignment for grading purposes, but separate assignment-side
Markdown would duplicate the canonical quiz source.

## Project assignments

The proposal, final project, and project submission are separate
assignments with overlapping requirements. They are preserved
in [the course-project directory](../course-project/) so that
educators can review and choose the material appropriate for their
course.
