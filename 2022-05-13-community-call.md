# GCC Rust Meeting 2022-05-13

- Date/Time: 13th May 2022 at: 09h00 UTC
- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: https://meet.jit.si/gccrs-community-call

## Agenda

- Project Reports
    - What are we working on
    - Testing project
    - Slices magic
    - Goal testcase
    - Metadata exports
    - i386 build issue
    - Build Badges
    - Maintaining cargo-gccrs
    - Rust edition poll on internals.rust-lang.org
- Questions

## Project Reports

Monthly report: https://github.com/Rust-GCC/Reporting/blob/main/2022-04-monthly-report.org

### What are we working on

philbert:

- Metadata exports
- Bugs

arthur:

- Privacy analysis
    - Reporting "private in public" errors and so on
- More path resolution

### Testing project

### Slices magic

I had issues when it came to debugging the compiled code generating a slice from an array. It boilds down to this function which was allocating FatPtr on the stack and this address won't exist:

```rust
struct FatPtr<T> {
    data: *const T,
    len: usize,
}

pub union Repr<T> {
    rust: *const [T],
    rust_mut: *mut [T],
    raw: FatPtr<T>,
}

const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {
    unsafe {
        Repr {
            raw: FatPtr { data, len },
        }
        .rust
    }
}
```

see: https://godbolt.org/z/KrxeYfxo9

From libcore 1.49.0
```
 262 #[inline]
 263 #[stable(feature = "slice_from_raw_parts", since = "1.42.0")]
 264 #[rustc_const_unstable(feature = "const_slice_from_raw_parts", issue = "67456")]
 265 pub const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {
 266     // SAFETY: Accessing the value from the `Repr` union is safe since *const [T]
 267     // and FatPtr have the same memory layouts. Only std can make this
 268     // guarantee.
 269     unsafe { Repr { raw: FatPtr { data, len } }.rust }
 ```
 
Thanks to @flip1995 
- https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=fbdf833b8adc64f4151752e2301db15b
- https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9873f9af2ad5620a77466797fae362ba
- https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=672adac002939a2dab43b8d231adc1dc

Turns out it is documented!

- https://doc.rust-lang.org/reference/dynamically-sized-types.html

```*const [T]``` is _not_ a pointer to the FatPtr struct. The language type of ```*const [T]``` is a dynamicaly sized type which is itself a struct passed by value which _is_ FatPtr itself.

see: https://github.com/Rust-GCC/gccrs/issues/1232

GDB must have some code here to know that taking the address of an array automatically make a slice structure.

### Privacy Pass

https://github.com/Rust-GCC/gccrs/pull/1246

@arthur TODO

### Goal testcase

https://github.com/Rust-GCC/gccrs/issues/682

Need support for range iterators and for loops.

@bjorn3 has also been testing with gccrs with the SIP hasher in libcore:

see: https://github.com/Rust-GCC/gccrs/issues/1247

### Metadata exports

This is in progress but I dedicated time to bugs and the goal-test case which has been really useful.

Looks like there are tests in GCC that assemble and link files together which will be really interesting way to do testing for this.

```
PASS: gcc.dg/lto/20081125 c_lto_20081125_0.o assemble, -O0 -flto -fuse-linker-plugin -fno-fat-lto-objects
PASS: gcc.dg/lto/20081125 c_lto_20081125_1.o assemble, -O0 -flto -fuse-linker-plugin -fno-fat-lto-objects
PASS: gcc.dg/lto/20081125 c_lto_20081125_0.o-c_lto_20081125_1.o link, -O0 -flto -fuse-linker-plugin -fno-fat-lto-objects
```

### i386 build issue

We were printing i64's using %i in some test cases which is not going to be platform agnostic. The other issue was that we did not respect host-wide-int when printing the array capacity mismatch errors.

- https://github.com/Rust-GCC/gccrs/commit/48cad9e8aed699c396afb7592dd637f2e87723dc
- https://github.com/Rust-GCC/gccrs/commit/dd213da6a3bd0587dc1e85a94674f935360ee5af

### Build Badges

- https://builder.sourceware.org/
- https://github.com/Rust-GCC/gccrs/issues/1236

### Maitaining cargo-gccrs

I plan on doing a full revamp of the architecture so that it's easier in the future to add flags, since gccrs is starting to support more and more.

### Rust edition poll

- https://internals.rust-lang.org/t/discussion-editions-in-rust-gcc-and-other-rust-compilers/16608/8

2015 seems to be heavily favored. What do people think?

## Questions

### Taking advantage of GCC

Take advantage of GCC tree's https://github.com/Rust-GCC/gccrs/issues/1204

We could add in a field to store the associated TyTy type to the tree for example:

```cpp
RUST_TY(TREE_TYPE(expr))
```

This would help make alot of code in the code-generation pass alot better. Lang fields like: 

```cpp
/* These flags are available for each language front end to use internally.  */
#define TREE_LANG_FLAG_0(NODE) \
  (TREE_NOT_CHECK2 (NODE, TREE_VEC, SSA_NAME)->base.u.bits.lang_flag_0)
#define TREE_LANG_FLAG_1(NODE) \
  (TREE_NOT_CHECK2 (NODE, TREE_VEC, SSA_NAME)->base.u.bits.lang_flag_1)
#define TREE_LANG_FLAG_2(NODE) \
  (TREE_NOT_CHECK2 (NODE, TREE_VEC, SSA_NAME)->base.u.bits.lang_flag_2)
#define TREE_LANG_FLAG_3(NODE) \
  (TREE_NOT_CHECK2 (NODE, TREE_VEC, SSA_NAME)->base.u.bits.lang_flag_3)
#define TREE_LANG_FLAG_4(NODE) \
  (TREE_NOT_CHECK2 (NODE, TREE_VEC, SSA_NAME)->base.u.bits.lang_flag_4)
#define TREE_LANG_FLAG_5(NODE) \
  (TREE_NOT_CHECK2 (NODE, TREE_VEC, SSA_NAME)->base.u.bits.lang_flag_5)
#define TREE_LANG_FLAG_6(NODE) \
  (TREE_NOT_CHECK2 (NODE, TREE_VEC, SSA_NAME)->base.u.bits.lang_flag_6)
```

Could be used for useful flags.

### GCC Git Branch

<https://github.com/Rust-GCC/gccrs/issues/143>

What if we create the Git branch by maintaining what the "mega-commit" will be to merge upstream.

Pros:

- Avoid rewriting the history to fix the bad commit message
- Avoid tweaking the git hooks

Cons:

- Looses history
- Doesn't resolve the general issue going forward (after the "mega-commit"): GCC/Rust development commit logs don't adhere to the standard GCC style
    - ..., and we probably don't want to enforce that as this point (proper ChangeLog updates for each Git commit!)

However, I (Thomas) have news to share about this topic, and also need discussion:

  - *devel* vs. *mirror*:
    Should GCC folks be permitted to push to the GCC-side *devel/rust/master*, and we occasionally merge that back into GitHub, or should the GCC-side branch in fact be just a read-only *mirror/rust/master*?
      - *mirror* would be a new Git branch hierarchy, with, basically, read-only semantics; just to make something visible on the GCC-side Git repository.
  - Also set up GCC Bugzilla?
      - Add new component "rust" at <https://gcc.gnu.org/bugzilla/editcomponents.cgi?product=gcc>.
      - Add new version "rust/master" at <https://gcc.gnu.org/bugzilla/editversions.cgi?product=gcc>.

### Tracking Contributors

We will be loosing our Git history when eventually merging upstream. Should we maintain a CONTRIBUTORS file in our front-end folder?
