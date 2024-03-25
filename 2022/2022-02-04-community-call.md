# GCC Rust Meeting 2022-02-04

- Date/Time: 4th February 2022 at: 14h00 UTC
- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: https://meet.jit.si/ArtificialPantsFlashNeither

## Agenda

- Project Reports
- What are we working on
- Questions

## Project Reports

- Monthly report: https://github.com/Rust-GCC/Reporting/blob/main/2022-01-monthly-report.org
- https://docs.google.com/spreadsheets/d/1B_JFzHgGclpdtPcQvnThkNJnP7Hh8fCIAU1rYFu_23M/edit?usp=sharing

## What are we working on

Philbert:
- CFG expansion got any, not and all working with -frust-cfg option
- About to start testing with the target options we get from GCC
- Key=value options https://doc.rust-lang.org/reference/conditional-compilation.html
- Adding more good-first-issues
- Going to spend time cleaning up the issues today

Marc (dkm):
- slowly continuing HIR visitor refactoring. Targeting a partial version during this week, hopefuly.

Arthur:
- GCC Self-test framework
- Joining Embecosm in a few weeks to work on GCC Rust

tschwinge:
- GCC merge upstream copyright headers and .c to .cc move

## Questions

### Overflow traps

Issue: https://github.com/Rust-GCC/gccrs/issues/404

Solution?
https://github.com/Rust-GCC/gccrs/blob/83bfbf0746c87b641754697a3c8e9f7a7cb08aa9/gcc/builtins.def#L796-L821

Can we use the `__BUILT_IN_{OP}_OVERFLOW` so we turn the operations into:

```rust=
let a:i32 = b + c;
```

```c=
i32 a;
if (__built_in_add_overflow(b, c, &a)) {
    // this will change to call the panic in libcore down the line
    __built_in_abort();
}
...
```

https://github.com/Rust-GCC/gccrs/blob/83bfbf0746c87b641754697a3c8e9f7a7cb08aa9/gcc/builtins.def#L824

https://doc.rust-lang.org/std/num/struct.Wrapping.html

### GTY and the TyTy nodes

In the typecheck pass we are creating new TyTy::<rust-type>(...) and storing them in side-tables for lookup down the line. GTY could cleanup all the memory for us.

https://gcc.gnu.org/onlinedocs/gccint/Inheritance-and-GTY.html#Inheritance-and-GTY
https://gcc.gnu.org/onlinedocs/gccint/User-GC.html#User-GC
    
It might be painful to get it working the GTY parser is a poor implementation of a C++ parser.
