# Rust GCC Meeting 2021-05-07

Second community video call on Zulip via jitsi

## Agenda

1. Planning Spreadsheet overview
2. Upcoming changes in the compiler
3. Compiler explorer
4. GCC Rust Mug
5. Code of conduct PR
6. Contributor guidelines
7. Philip's vacation time
8. Questions

## Planning Spreadsheet Update

https://docs.google.com/spreadsheets/d/1B_JFzHgGclpdtPcQvnThkNJnP7Hh8fCIAU1rYFu_23M/edit?usp=sharing

Not much change, Generics to be finished soon and Philip will be moving onto traits which is the big scary one. Philip will not break the build but there might be issues with merge conflicts if you are working in the type system area.

## Upcoming changes in the compiler

```rust
pub fn test() {
    let a = return;
    let b = a + 123; // no impl for () + <i32>
}
```

vs 

```rust
pub fn test() {
    let a = return;
    let b = a + 123;
    a = 123;
}
```

This requires the type system in the compiler to split up type inference and type checking. The goal here is that the type system becomes 3 passes:

1. Type inference
2. Trait solving
3. Type checking

## Compiler explorer

https://godbolt.org/

Thanks for all the work here from Marc, we are not at the bottom of the rust list which helps hide the fact we are not stable yet.

## GCC Rust Mug

Philip will let you know when your mug is dispatched

## Code of conduct PR

https://github.com/Rust-GCC/gccrs/pull/372

We can just merge this for now but change the contact details

## Contributor guidelines

https://github.com/Rust-GCC/gccrs/pull/373

This is shaping up well lets give it a week or so to finalize the merging details, but we can always change this over time.

## Philip's vacation time

17,18,19 May

## Questions

### What about the rlib metadata in rustc?

GCC Go emits declarations into a section within the object code files and objdump is used to extract this into .gox files which can be parsed to figure out all the declarations.
See https://github.com/Rust-GCC/gccrs/blob/master/gcc/go/go-backend.c

There are other options such as TREE/LTO streaming and gimple dumps (?) etc. which might also fit.
Also mentioned: the new C++ modules' implementation.


### Overflow behaviour

We need to figure out the behaviours for overflow, GCC has multiple implementations we can use to handle this.
See https://github.com/Rust-GCC/gccrs/issues/404


### https://github.com/Rust-GCC/gccrs/issues/412 "GCC Abstractions"
