# GCC Rust Meeting 2021-11-05

- Date/Time: 5th November 2021 at: 15h00 UTC
- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: https://meet.jit.si/259057065581073

## Agenda

* Project Status
* Questions

## Project Status

Monthly Report for October 2021: https://github.com/Rust-GCC/Reporting/blob/main/2021-10-monthly-report.org

- Closures blocked lang-items on the way
- Philip am working on a blog post about 1 year of development on GCC Rust
- Philip has been focued on bugs for the goal-test case: https://github.com/Rust-GCC/gccrs/issues/682
- flip1995 has forked Blake3 to remove the imports and allow this to compile with no_std and no_core 
- Philip is on vacation next week but will likely still be online

## Good First PR's

New list of good first PRs which contain tasks guides please give feedback: https://github.com/Rust-GCC/gccrs/issues?q=is%3Aissue+is%3Aopen+label%3Agood-first-pr

- More to be added
- Tutorial for the builtins to be done

## Clang-Format

Do we want to keep automated formatting. No clear consensus.

Problems:

1. GCC Upstream is not compliant to clang-format (yet?)
2. clang-format-10 to clang-format-11 has subtle changes
3. Some clang-format changes are bad see https://github.com/Rust-GCC/gccrs/pull/779

Solutions

1. Add a git pre-commit hook for clang-format to enforce every commit is clang-formatted
2. Stop enforcing clang-format on PRs
3. Point developers to clang-format and gcc-coding standards

Issue created: https://github.com/Rust-GCC/gccrs/issues/797

## Splitting up work

We have github issues and projects to see what Philip is working on look at the kanban board for the current milestone: https://github.com/Rust-GCC/gccrs/projects/12

1. Philip will send a weekly email on Github detailing his planned work each week
2. Each week this might encourage others to contribute and say what they are working on

Philip: Working on bugs and lang-items
Marc (dkm): Working on HIR dumps
Mark (mjw): Working on https://blog.rust-lang.org/2021/11/01/cve-2021-42574.html and GCC diagnostics

## Questions

### Github issue about builds

https://github.com/Rust-GCC/gccrs/issues/782

This is a question which can be closed out, we need to ensure our README.md points out that to build GCC you should build outside of the GCC dir.

### Meeting time

We will attempt to move the meeting to 1400 UTC next month.
No active contributors in Asian etc. time zones right now, but in the U.S.A.

### JSON diagnostics

"Should work", GCC has some support at least.

Share error codes with `rustc`?

`compiler/rustc_error_codes/src/error_codes.rs` 

Currently not doign error codes in GCC, but could (at least in the GCC/Rust front end)?

Not always a 1-to-1 correspondence between implementations?

### Better distinguish between user-level errors and internal (consistency etc.) checks.

Use `rust_error_at` etc. for the former (and test cases!), and `rust_assert`/`rust_internal_error` for the latter. This will help cleanup the code over time to distingush internal errors vs program errors.

