# GCC Rust Meeting 2021-10-02

- Date/Time: 1st Oct 2021 09h00 UTC
- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: https://meet.jit.si/259057065581073

## Agenda

* Project Status
* Getting involved
* Time slot
* GCC Merge
* GCC Bootstrap

## Project Status

Monthly Report for September 2021: https://github.com/Rust-GCC/Reporting/blob/main/2021-09-monthly-report.org

New timeline included with planning spreadsheet:

https://docs.google.com/spreadsheets/d/1B_JFzHgGclpdtPcQvnThkNJnP7Hh8fCIAU1rYFu_23M/edit?usp=sharing

There is a new estimated timeline for the project within this planning speadsheet

## Good First PR's

New list of good first PRs which contain tasks guides please give feedback: https://github.com/Rust-GCC/gccrs/issues?q=is%3Aissue+is%3Aopen+label%3Agood-first-pr

## This meetings time slot

I propose moving this meetings time slot to 1600UTC for every 2nd meeting to allow for people in the US timezones to join.

## GCC Merge

Thanks to Thomas for the GCC merge from upstream everything seemed to go well. Were there many merge conflicts? Marks build farm is green so far.

## Working GCC Bootstrap

This took quite a few PR's to fix: https://github.com/Rust-GCC/gccrs/issues/336

Do we want to enable -Werror checks for all PRs? https://github.com/Rust-GCC/gccrs/issues/694

## Compiler intrinsics

Lots of work to share out, lets make a tutorial on this:

https://github.com/Rust-GCC/gccrs/issues/658
