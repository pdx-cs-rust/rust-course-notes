# Quizzes

Quiz files contain portable teaching material, including public
answer keys and feedback. Course-specific identity and deployment
state live in the ignored `.canvas/` directory.

The filename identifies the local quiz. `rust-quiz.md` and
`advanced-rust-quiz.md` preserve each existing quiz's wording,
answer order, scoring, and feedback. Formatting changes turn
HTML paragraphs, lists, and code into Markdown. Existing wording
and content errors remain intact.

## Markdown convention

Use one level-one heading for the quiz title. The `Settings`
section contains pedagogical settings as key-value list items.
The `Instructions` section contains the introduction students
see before answering.

Each `## Question N` begins a question, in presentation order.
Its metadata list gives the original name, portable type, and
point value. The `### Prompt` section contains the question.

Each `### Answer N` begins an answer, in presentation order.
Each answer uses `Weight`, the percentage of the question's
available credit assigned to that answer. Preserve numeric
weights exactly, including intermediate values. The `#### Text`
section holds the answer content. An optional `#### Feedback`
section holds its feedback; absence means no feedback. Answer
keys and feedback are public on purpose.

For `multiple choice`, students select one answer. Each such
question in `rust-quiz.md` has exactly one answer weighted 100
and all other answers weighted 0. `rust-quiz.md` has 15
questions and 54 answers; `advanced-rust-quiz.md` has seven
questions and 32 answers. Both quizzes have 100 points.

Headings used by this convention are structural. Use other
heading levels or a fenced code block when a prompt needs to
quote one. Rust snippets use fenced `rust` blocks; type-only
answers use inline code so angle brackets survive rendering.

## Recovery

The Markdown records instructional content and portable quiz
behavior without platform identifiers, dates, or publication
state. The ignored local overlay preserves Canvas identity and
deployment state. Keep a backup of `.canvas/` alongside this
repository until the course-directory integration is arranged.

A reader can reconstruct the quiz content from the title,
instructions, ordered questions, answer keys, scoring, and
feedback. The original quiz should be exported again before
redeployment if its live content has changed.
