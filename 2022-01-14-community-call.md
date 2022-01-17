# GCC Rust Meeting 2022-01-14

- Date/Time: 14th Janurary 2022 at: 14h00 UTC
- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: https://meet.jit.si/ArtificialPantsFlashNeither

## Agenda

- Project Reports
    - Goal for 2022
    - Goal Testcase Blake3
- What are we working on
- Get Involved
- GCC GSoC 2022: Call for project ideas and mentors
- Questions

## Project Reports

- Rust in 2021: https://github.com/Rust-GCC/Reporting/blob/main/2021-year-report.org
- Weekly report: https://github.com/Rust-GCC/Reporting/blob/main/2022-01-10-report.org
- https://docs.google.com/spreadsheets/d/1B_JFzHgGclpdtPcQvnThkNJnP7Hh8fCIAU1rYFu_23M/edit?usp=sharing

### Goals for 2022

Philbert:

- Macro expansion
- Cfg expansion
- Imports (use declarations)
- Visibility modifiers
- Const generics
- Bugs

Once we get though the macros milestone we should be moving onto trying to compile libcore as a new goal testcase to find more bugs.

### Goal Testcase

https://github.com/Rust-GCC/gccrs/issues/682

We are missing support for Slices and Ranges, many bugs have been found by testing this so far.

## What are we working on

Philbert:

- GSoC 2022 wiki updates
- Const folding
    - see https://github.com/Rust-GCC/gccrs/pull/870
    - cpp constexpr -O0 https://godbolt.org/z/rjMYGhrMd
- Method resolution
    - Needs to support calling into deref operator overloads
- Planning Macros milestone
    - Rustc has its own parser for macro's I am hoping we could reuse our existing parser
        - The token stream contains spacing information aparently
    - Need to plan out tickets
    - Testing could be done by comparing AST Dumps vs the expanded rustc

Marc P:
 - HIR refactoring https://github.com/Rust-GCC/gccrs/issues/825 . But currently not moving much (sorry).
## Get Involved

Not all development needs to be PR's we acively are seeking testers to raise issues. You can easily test our compiler via:

- Compiler Explorer: https://godbolt.org/z/Whq8YM7EP
- Docker: https://hub.docker.com/r/philberty/gccrs

We have a list of good first issues which contain a guide and extra information. Please feel free to join our Zulip/irc/mailing-list to ask for more guidence/mentorship on any issue.

- Good first issues: https://github.com/Rust-GCC/gccrs/issues?q=is%3Aissue+is%3Aopen+label%3Agood-first-pr

Our contributor guide has more useful information: https://github.com/Rust-GCC/gccrs/blob/master/CONTRIBUTING.md


## GCC GSoC 2022: Call for project ideas and mentors

  - <http://mid.mail-archive.com/ri6czl4x2d9.fsf@suse.cz>
      - <https://gcc.gnu.org/wiki/SummerOfCode>
  - <https://github.com/Rust-GCC/gccrs/wiki/Google-Summer-of-Code>

## Questions

### How do we get our branch onto GCC

Can someone help with this, we hit: https://github.com/Rust-GCC/gccrs/issues/143

mjw: has root access to sourceware
tschwinge: will look into this for us

We need to consider what the home of gccrs development is down the line.
