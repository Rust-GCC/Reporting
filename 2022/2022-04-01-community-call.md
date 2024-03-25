# GCC Rust Meeting 2022-04-01

- Date/Time: 1st April 2022 at: 14h00 UTC
- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: https://meet.jit.si/gccrs-community-call

## Agenda

- Project Reports
    - What are we working on
    - Metadata exporting
    - Testing project
    - Borrow checking investigation
    - Crash on huge arrays
- Questions

## Project Reports

Monthly report: https://github.com/Rust-GCC/Reporting/pull/21

### What are we working on

philbert:

- Metadata exporting research
- type system bugs
    - generic higher ranked trait bounds
    - referencing associated types from higher ranked trait bounds
    - bad overlapping impl checks [T] vs T 

arthur:

- Figuring out the various privacy/visibility visitors
    - rustc_privacy pass
- How do we implement them, where do we add them
    - After typechecking, in the HIR
    - There's actually a type privacy and a general item privacy (Thanks @flip1995)
    - How to handle macros being exported in the root and maybe leaking private functions
- Work on testing project
    - Runs gccrs on the rustc tesuite with -fsyntax-only and compare results with rustc for syntax only
    - Runs rustc against our dejagnu testsuite

tschwinge:

- gcc/rust/lang.opt disturbs other GCC front ends
- working on merge upstream

dfaust:

- builtin macros include_str and include_bytes
- gcc garbage collector with type-system

### Metadata exporting

- LTO streaming
    - LTO is not supported on all platforms
        - This probably means LTO functionality isn't supported by all linkers -- but the LTO/TREE streaming that we need should be generic, supported everywhere.

- CPP module code
    - specific to CPP
    - does contain its own codec for GCC tree's
    - tied in with CPP lang-specific trees
    - Still unclear why this doesn't build on LTO streaming
        - Maybe it's a different abstraction level: C++ front end AST etc. vs. LTO: GIMPLE/TREE?

- Roll our own
    - I think we can reuse parts of cpp module code to export type'd prototypes
    - We need to be able to export macros, traits (which are very abstract, not tied to GCC tree's)

Still need to investigate the contents of rustc's metadata.

### Testing project

https://github.com/Rust-GCC/testing

- Testing GCC Rust in the wild
    - Test parsing -fsyntax-only on rustc testsuite
    - Test rustc against gccrs dejagnu testsuite
        - most failures are because we have not implemented the main-shim
    - Test gccrs against rustc fully
    - Test gccrs against projects like https://github.com/Rust-GCC/gccrs/issues/682
    - Benchmarking
    - code-generation comparison and research
    - ... to the moon

This project will allow us to automate more complex testing outside of the compiler project. We can automate and track the results over time for every merge to master on gccrs.

### Borrow checking investigation

https://hackmd.io/msucyEdfRHeJr2ykuqKF2g

We have completed an investigation into borrow checking with polonius. I believe using polonius has several main advantages:

- Contribute back to the rustc eco-system
- Maximize compatability with rustc
    - rustc aims to reuse this too

### Crash on huge arrays

Dfaust has a nice solution to get the gimple pass to generate a memset call like rustc.

https://github.com/Rust-GCC/gccrs/issues/1068

rustc still crashes on a const/static context
https://godbolt.org/z/TWznxnfon

## Questions


```
M V V S Manoj Kumar
M V V S Manoj Kumar says:Hi, I was planning on a project proposal on constant folding and was exploring PRs of the old mapping improvements, but I guess I'll take a bit more time to explore gimple and how the c++ frontend does the optimization, and then I'll be in a position to write a proposal. Could you guys point me in a direction how the rustc implements it so I can look at it too? 

Also, I was interested in the dead_code pass project too. Could you give me a few prs I could look at and see how it goes about it? or how i could get on it. 
M V V S Manoj Kumar says:sure that would be helpful
```

Addressed post-call with more info and examples on zulip
