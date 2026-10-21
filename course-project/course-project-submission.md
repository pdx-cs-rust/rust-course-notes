# Course Project Submission

Please submit your final project repo URL. I strongly prefer
GitHub. Ideally this is the same URL you submitted for the
proposal; drop me a note on Zulip if it's different so that I'm
sure to get the right one.

## Course Project Requirements

The project parameters and policies stated in the Course Project
Proposal continue to apply, including the theme or your approved
theme opt-out, individual or approved team work, and the rules on
reusing projects. In particular:

- One or more Rust crates. The project can be a binary crate, a
  library crate or some combination.

- The project should perform a coherent function. Roughly
  500-1500 lines of code, but smaller can be fine: if you're
  gonna go larger ask me. Line count is a stupid measurement
  anyhow.

- Code should be written in a reasonable modular style. Code must
  compile without warnings and must not have `rustfmt` or
  `clippy` errors. The codebase should be largely free of
  commented-out code.

- Include internal unit tests where needed to check correctness.
  Adequacy of testing is left to your judgement.

- Items in crate public interfaces must have good `rustdoc`.
  Include additional developer documentation if needed to help
  me understand your software.

- The project must build using stable Rust, unless otherwise
  approved. `unsafe` code should be kept to an absolute minimum
  and clearly documented for both safety and need.

- Projects must consist mostly of code you wrote yourself.
  Using established models, rules, and algorithms is fine;
  copying an existing implementation or closely following a
  tutorial is not. Describe your own contribution and
  acknowledge any sources. The usual plagiarism rules apply.

- Follow the proposal's guidance on tutorials and AI. I prefer
  that you not write Rust with AI; using AI for ideas and
  algorithms is fine. Thoroughly document any tutorials or AI
  used in your README.

- The project commit history must accurately reflect project
  development.

## README

Your project must include a `README.md` file in Markdown format
containing:

- Your name and the project name.
- A description of what the project does and how it works
  (a paragraph or two).
- Detailed instructions for building and running the project.
- How you tested the project.
- An example illustrating the operation of your code.
- What worked, what didn't, and what lessons you learned.
- How satisfied you are with the result and what you would like
  to improve in the future.
- Your own contribution and acknowledgements of sources,
  tutorials, and AI assistance, as applicable.
- License information, if any, including pointers to the license
  files.

## Repository

The repo must be cloneable by `BartMassey`. I strongly prefer
that you make it public: check this using an incognito window or
something. If you want to keep it private, contact me on Zulip
so that we can make sure the TA and I can access it.

If the repo is public, include the text of an appropriate open
source license in the root directory, not just a link. I
recommend MIT + Apache 2.0 for Rust projects.

Do not check junk files into the repo: build artifacts (including
`target/`), editor and IDE junk, backup files, or old copies of
the project. IDE project files needed to build the project are
fine. Someone cloning the repo should get what they need to build
and work on your code.

## Video Presentation (optional)

Your project repository may contain a video presentation, no more
than three minutes long. Name it `PRESENTATION.zzz`, where `zzz`
is your video format (for example, `PRESENTATION.mp4`).

Give your name, the project name, and a brief description of what
the project is and does. A super-quick demo of some functionality
would also be great. Discuss the project's limitations and any
future plans.

## Submission

Submit your project's repo URL to this assignment on Canvas,
even if it is the same URL you submitted for the proposal. Make
sure that the repo is publicly readable or that you have arranged
access with me as described above.
