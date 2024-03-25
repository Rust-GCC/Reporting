# Rust GCC Meeting 2021-06-04

Second community video call on Zulip via jitsi: https://meet.jit.si/259057065581073

## Agenda

* Monthly report overview
    * Planning Spreadsheet overview
* Builds on different architecture's
* Merging GCC upstream
* GCC Copyright assignment and us
* GCC Mailing List and IRC
* Google Summer of Code 2021 projects

## Monthly report overview

Monthly Report for May 2021: https://github.com/Rust-GCC/Reporting/blob/main/2021-05-monthly-report.org

The traits milestone will have progress made by Monday; if Philip gets the initial building blocks right, the rest of the milestone will go smoothly; otherwise, we risk rewriting iterating on obligations are managed.

This milestone is all about completing the type system.

### Planning Spreadsheet Update

https://docs.google.com/spreadsheets/d/1B_JFzHgGclpdtPcQvnThkNJnP7Hh8fCIAU1rYFu_23M/edit?usp=sharing

This is about Philip Herron's focus each month.

## Builds on different architectures

Recently Mark Wielaard has created a buildbot infrastructure for GCCRS: https://builder.wildebeest.org/buildbot/#/builders?tags=gccrust
and John Paul Adrian Glaubitz manually tested further configurations and supplied a few small patches.

Thanks for maintaining the Target Info stuff, this is not clear why it is used yet, but both Arthurs Google Summer of Code project depends on this and configuration expansion needs this as well; both are in progress.

## Merging GCC upstream

This is done periodically; our front-end code is agnostic of the changes in general, so it is not a priority, but we should try to keep up to date to avoid awkward conflicts.

## GCC Copyright assignment and us

* https://gcc.gnu.org/pipermail/gcc/2021-June/236182.html
* https://github.com/Rust-GCC/gccrs/issues/461

We have no plans to change our policy of Copyright assignment for now. When the dust settles, we can figure out what changes we need to make.

## GCC Mailing List and IRC

* GCC Mailing List: https://gcc.gnu.org/mailman/listinfo/gcc-rust
* irc: irc.oftc.net - gccrust

The mailing list will be in addition to Zulip supplying the traditional communication methods for the project and notifying those with status reports etc.

## Google Summer of Code 2021 projects

Both students are progressing nicely. Thank you for your hard work.

### Arthur - cargo support for GCCRS.

* Hosted on: https://github.com/Rust-GCC/cargo-gccrs 
    - This is started and is finding issues with GCCRS already.
    - It is consuming the Target Info see https://github.com/Rust-GCC/cargo-gccrs/issues/1
    - The Goal for the compilehttps://hackmd.io/rBFlwl_9TkWLox-X6jyxDgr perspective here is to have cargo build tool support
        - To find missing flags and features from the compiler.
        - This feeds into the compiler imports and visibility milestone

### Thomas - unused/deadcode

* Developed within the GCCRS compiler itself
    - Already has some code in the compiler
    - Currently developing out how Attributes are handled
    - Has a kanban for progress: https://github.com/Rust-GCC/gccrs/projects/9
    - The goal here is for the compiler to improve its general diagnostics
        - This means a framework for Attributes in general ("allow_deadcode")
        - finding unused fields within a struct or mutability

