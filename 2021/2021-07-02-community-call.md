# Rust GCC Meeting 2021-07-02

- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: https://meet.jit.si/259057065581073

## Agenda

* Monthly report overview
    * Planning Spreadsheet overview
* Google Summer of Code 2021 projects
* Error handling
* Code Review
* Technical Debt
* Unit Type

## Monthly report overview

Monthly Report for June 2021: https://github.com/Rust-GCC/Reporting/blob/main/2021-06-monthly-report.org

Interesting reading, which is what I am using as a reference.

- https://rustc-dev-guide.rust-lang.org/traits/resolution.html
- https://rustc-dev-guide.rust-lang.org/traits/chalk.html

### Planning Spreadsheet Update

https://docs.google.com/spreadsheets/d/1B_JFzHgGclpdtPcQvnThkNJnP7Hh8fCIAU1rYFu_23M/edit?usp=sharing

This is about Philip Herron's focus each month. No change though come closer to end of July mid August features such as pointers and the rest of the type system should be in progress.

Its hard to say if Traits are on track or not as implementing the support for TraitBounds and Where constraints are complex and its taking time to get right.

## Google Summer of Code 2021 projects

Both students are progressing nicely. Thank you for your hard work.

Both are on track for mid-July to complete their GSOC goals. Although each project has limitations due to the lack of features in GCCRS, the focus should be on maintainability for the code. They are easily extensible in the future.

### Arthur - cargo support for GCCRS.

Monthly Report: https://github.com/Rust-GCC/Reporting/blob/main/2021-06-gsoc-2021-arthur-cohen-monthly-report.md
    
### Thomas - Deadcode analysis

Monthly Report: https://github.com/Rust-GCC/Reporting/blob/main/2021-06-gsoc-2021-thomas-young-monthly-report.org

## Error Handling

We should start taking advantage of type check TyTy::Error and of GCC's error_mark_node. The pipeline executor in rust-session-manager.cc each pass checks for saw_errors() and exits if any errors have been emitted. This was a stop-gap measure to get the compiler off the ground; we are getting to a level of maturity that this no longer makes as much sense. Most of the work involved in fixing this issue will be updating the back-end code to no longer emit rust_fatal_error for nullptr on trees but use error_mark_node instead, which will let GCC error out appropriately.

https://github.com/Rust-GCC/gccrs/issues/539

## Front-end Errors

Typechecking errors will be different in Rust vs C++ (before/after monomorphization).

We have been trying to implement major error checks from Rust which helps reinforce confidence in the implementation details within the code.

## Code Reviews and PRs

Contributing guidelines: https://github.com/Rust-GCC/gccrs/blob/master/CONTRIBUTING.md

Philip would apreaciate any help with code review, the project is in an ealy phase where so long as a PR does not add any regressions to the test-suite and has a linked issue that is a missing feature or bug I am happy for anyone else with bors rights to merge this.

## Bors rights holders

- Philip Herron [philberty](https://github.com/philberty/)
- Joel [SimplyTheOther](https://github.com/SimplyTheOther)
- Marc Poulhiès [dkm](https://github.com/dkm)
- Thomas Yonug [thomasyonug](https://github.com/thomasyonug)
- Thomas Schwinge [tschwinge](https://github.com/tschwinge)

Mark Wielaard deserves bors rights but you need a github account ;).

## Technical Debt

HIR was bootstrapped from the AST which lead to a lot of duplication. As of https://github.com/Rust-GCC/gccrs/pull/540 any HIR::Method reference no longer exists. In order to address issues with technical debt the best place to start is by cleaning up the IR's as this drives the code changes in general. The goal is to make sure HIR is immutable so we can start using const to improve the apis in general.

Feel free to code review the compiler and suggest any ideas to help with the code structure in general or mention any pain points.

## Unit Type

The current situation with unit type is that we are using void_type_node as the return type on functions but a zero precision integer for the actual type every where else including unit-structs, though we might experience with an empty Record type for unit structs. This could be used as a way to reach out to the rustc_gcc_codegen project to see what issues they have found implementing unit-type.
