# `cargo-gccrs` - Monthly report June 2021

- Name: Arthur Cohen
- Github Profile URL: [https://github.com/cohenarthur](https://github.com/cohenarthur)

## Overview

The goal of `cargo-gccrs` is to allow rust's building system, [cargo](https://github.com/rust-lang/cargo),
to use `gccrs` as an alternative compiler to `rustc`.
In order to allow the rust language to target more of the current architecture ecosystem,
as well as help in resolving the bootstrapping problem, rust support for the GNU Compiler
Collection is currently being developed. The compiler is getting more and more usable
for rust code in general, and could definitely benefit from more usage from the community.
Adding support for `gccrs` to the `cargo` build system would be a stepping stone in having
more of rust users test the compiler and use it.

## Progress Report

### Achieved functionality

Work on `cargo-gccrs` started at the beginning of the month of June. In that time, we have
managed to allow the compilation of simple projects, in order to generate correct binaries
in the formats of an executable, a shared library or a static library.

We have designed an option parser that is able to translate `rustc` options into valid
`gccrs` options. The program then spawns a `gccrs` command with valid arguments in order
to compile the given rust project.

We added a few basic projects to the repository, that are compilable using `gccrs`
(this means only one file, no rust macros, etc). We also worked on incorporating a testing
framework to make sure that `cargo-gccrs` can compile those basic projects as it is being
tweaked and worked on.

If your project is simple enough, you are now able to use `cargo-gccrs` for building and
running it. Simply use `cargo gccrs build` or `cargo gccrs run`.

### Problems encountered

__`gcc` cannot produce multiple binaries with one command__:

The rust compiler is often invoked by cargo with the goal of generating a binary, a static
library and a shared library. This is achieved by passing multiple instances of the `--crate-type`
argument, which `gcc` does not have an equivalent for. Therefore, `cargo-gccrs` might have
to resort to spawning multiple `gccrs` commands in order to reproduce the behavior of `rustc`.

__`gcc` cannot directly produce static libraries__:

`rustc` can directly create archive files containing multiple object files. On Unix, this
corresponds to the `.a` file format, and these archives are created using the `ar`
command. `gcc` does not have any means to do that, and `cargo-gccrs` has to spawn a secondary
command using the `ar` program, whose aim is to package a previously created compilation unit
generated using `gccrs`.

__`gccrs` cannot generate the expected filenames of binaries__:

When displaying the compiler's configuration, `rustc` can print the expected output filenames
of different binaries. For example, on Unix systems, a shared library shall be called
`lib<name>.so`. A binary can simply be called `<name>`, and a static library will usually
be called `lib<name>.a`. This being displayed is one of the requirements for `cargo` to
work with the compiler. As this isn't implemented yet, an issue has been opened
[here](https://github.com/rust-GCC/gccrs/issues/490) to discuss it.

This work was achieved thanks to the thoughtful reviews and contributions from
[@philberty](https://github.com/philberty), [@flip1995](https://github.com/flip1995) and
[@bjorn3](https://github.com/bjorn3). Thanks a lot for your implication!

## Goals for this GSoC

The goals for this GSoC were as follow:

- Create a barebones rust project as a cargo subcommand
- Get familar with `gccrs` internals
- Build initial `cargo-gccrs` with basic functionality
- Investigate `cargo` in depth in order to replicate `rustc`'s behavior as closely
as possible
- Allow for users to compile all sorts of binaries
- Add abstractions around testing in order for the project to benefit from the large
amount of rust code already written
- Report any kind of issue with the compiler to the main `gccrs` repository
- If time is left, work on fixing those issues directly in the compiler.

The project is progressing nicely. We are on track to having most of the functionality
implemented, cleaned-up and well tested. A lot of work still remains, and a lot of issues
are yet to be discovered. I should have some time to dedicate to contributing directly to
`gccrs` in order for `cargo-gccrs` to be even more useful. I think that I am on track to
complete a minimum viable product for this GSoC project, which could then be easily
maintained and improved upon as the compiler progresses.

## Status Table

|Category    | Last Month | This Month | Delta|
|------------|------------|------------|------|
|Todo        |   0        |   17       | +17  |
|In Progress |   0        |   3        | +3   |
|Completed   |   0        |   6        | +6   |
|Blocked     |   0        |   0        | 0    |

## Risks

The `gccrs` compiler does not implement all of the functionality that the `rustc` compiler
provides. For example, a lot of rust projects use multiple files, which are not handled
yet. This reduces the usability of the tool as it is not yet able to compile multiple
projects. In the same vein, macros and procedural macros are not supported yet: Which means
that implementing `cargo gccrs test` is still a long way off, as `rustc` relies on the
`libtest` rust library for testing, which relies heavily on those macros
(`#[test]`, `#[test_runner]`, `assert!`, etc).
