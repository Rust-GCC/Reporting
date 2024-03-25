# GCC Rust Meeting 2021-12-03

- Date/Time: 3rd December 2021 at: 14h00 UTC
- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: ~~https://meet.jit.si/259057065581073~~ https://meet.jit.si/ArtificialPantsFlashNeither

## Agenda

- What are we working on
    - Project Status
    - Spliting up work
- Questions

## What are we working on

- philbert:
    - Pattern Matching
    - Enum code generation
    - Raising issues to refactor the code base to make it easier to work with
    - Working on a blog post reflecting on a year of development so far
- Marc P:
    - HIR visitor refactoring
    - Better HIR dump
- Mark W: 
    - V0 symbol demangling in GCC
    - https://gcc.gnu.org/pipermail/gcc-patches/2021-December/586058.html

### Project Status

Monthly Report for November 2021 PR: https://github.com/Rust-GCC/Reporting/pull/5

- Lang-items
- Operator overloading
    - Autoderef and dereference operator overloading
- Bug Fixes: https://github.com/Rust-GCC/gccrs/issues/682

### Splitting up work

1. Assigning issues directly to people when they are interested 
2. Raising more github issues with guides to share knowledge.
3. Reach out on zulip/github-issues/irc to ask about issues

## Questions

### ADT's in gimple dumps

Currently all ADT's get lowered to RECORD or UNION types in GENERIC and then to gimple but even though we are setting TYPE_NAME to the TYPE_DECL the name is empty in -fdump-tree-gimple the debug_tree output seems to be correct also.

### Method Resolution Candidate search

Does searching for candidates in method calls require autoderef to filter what impl block might be applicable. 

https://rustc-dev-guide.rust-lang.org/method-lookup.html

Looks like the documentation has been updated detailing the Probe phase now.
