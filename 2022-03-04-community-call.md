# GCC Rust Meeting 2022-03-04

- Date/Time: 4th March 2022 at: 14h00 UTC
- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: https://meet.jit.si/ArtificialPantsFlashNeither

## Agenda

- Project Reports
    - What are we working on
- Questions

## Project Reports

- https://github.com/Rust-GCC/Reporting/blob/main/2022-02-monthly-report.org

### What are we working on

- Philbert
    - Slices
        - This relies on lots of libcore code which is very hard to follow
        - see at the end of the notes for example
            - This is how rustc <= version 1.49.0 implements it
    - Fixing unused warnings
        - rust must_use = CPP nodiscard
        - cleans up our TREE_USED usage
        - cleans up our deadcode pass to respect gcc -Wunused-functions
    - Dyn traits must support super trait call's
        - I have it working locally
        - Selecting vtable ptr's needs some thought on edge cases

- Arthur
    - Fixing remaining issues on macro repetitions
        -  proper comma expansion, error messages and so on
    - Rules for token following repetition patterns
        - `($($e:literal -> )*)` vs `($($e:literal) -> *)`
        - Only a single token is allowed *after* the parentheses, but multiple tokens are allowed inside them (inside the repetition pattern)
        - `($($e:literal some whole lot of toks)*)` is all good
    - Fixing edge cases for macro repetitions
    - Implementing macro builtins

- Marc
    - HIR Visitor refactoring Merged!!! :D
    - Will give another shot at HIR dump (or will have a look at enum/ada stuff)

- Mark:
https://gcc.gnu.org/pipermail/gcc-patches/2022-February/590551.html

buildbots

- Thomas:
valgrind self-test wrapper

## Questions

### What is the difference in .rlib and .rmeta

Rustc outputs the compiled artefact along side .rlib and .rmeta during compilation of a crate what do these contain?

gccgo embeds the public members of a .go file into .go_export of each object file and objtool then dumps these into .gox files which act as a kind of header file for gccgo.

We can also embed information similar to LTO. Though rust needs the ability for us to export public generic functions/traits/structures so quite detailed information needs to be available for us so GCC tree's may not be suitable for this.

------

LTO streaming is version dependant

```
bjorn3
bjorn3 says:Correct. The current implementation of .rlib is indeed a .a archive. 
bjorn3 says:rustc -Zls prints a tiny bit of info. I once wrote a partial decoder, but it doesn't work for current rustc versions anymore. 
bjorn3 says:
https://github.com/bjorn3/rust_read_rlib
 
bjorn3 says:depends on rustc 1.35.0-nightly (fbd34efb3 2019-03-26) 
bjorn3 says:MIR is an implementation detail. It is unstable. 
bjorn3 says:The full metadata format is unstable and somewhat regularily changes. 
14:25
mjw
mjw says:so similar to gcc LTO then 
14:26
bjorn3
bjorn3 says:I don't think cross-compiler linking is feasible for rust. At least not in the near to middle term. 
bjorn3 says:Too much things are unstable. 
bjorn3 says:
https://github.com/rust-lang/rust/pull/94570
 which is currently being tested for example changes the rust abi. 
bjorn3 says:one advantage (for some people) of keeping the metadata format unstable is that it pretty much forces rust libraries to be open source. 
```

C++ modules support might be good idea to look into

### GCC -Wuninitialized -Wmaybe-uninitialized

GCC has lots of really great static analysis and in rust any read of and uninitialized variable is an error, but this is a warning in GCC. Is there a way to make these an error?

-Werror makes all warnings error which isn't right as unused code is not an error in rust.

Thomas Schwinge:

You may specify selective '-Werror=uninitialized' etc.

There's the usual caveat that '-Wuninitialized' diagnostics are dependent on optimization level, and in particular '-Wmaybe-uninitialized' may produce false positives.
..., and thus '-Werror=[...]' may inappropriately halt compilation.
Assuming that Rust has precise rules about what constitues "uninitialized" state, there should probably be GCC/Rust front end logic to trace that (and then possibly feed the common GCC "uninitialized" diagnostics machinery with that information (by setting appropriate flags on the TREEs etc.


---

needs investigation into optimization levels


### Rustc slice implementation

```rust=
extern "rust-intrinsic" {
    pub fn offset<T>(dst: *const T, offset: isize) -> *const T;
}

#[lang = "index"]
trait Index<Idx> {
    type Output;

    fn index(&self, index: Idx) -> &Self::Output;
}

pub unsafe trait SliceIndex<T> {
    type Output;

    unsafe fn get_unchecked(self, slice: *const T) -> *const Self::Output;

    fn index(self, slice: &T) -> &Self::Output;
}

#[lang = "Range"]
pub struct Range<Idx> {
    pub start: Idx,
    pub end: Idx,
}

union Repr<T> {
    rust: *const [T],
    rust_mut: *mut [T],
    raw: FatPtr<T>,
}

struct FatPtr<T> {
    data: *const T,
    len: usize,
}

impl<T, I> Index<I> for [T]
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}

#[lang = "const_ptr"]
impl<T> *const T {
    pub const unsafe fn offset(self, count: isize) -> *const T {
        unsafe { offset(self, count) }
    }

    pub const unsafe fn add(self, count: usize) -> Self {
        unsafe { self.offset(count as isize) }
    }

    pub const fn as_ptr(self) -> *const T {
        self as *const T
    }
}

const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {
    unsafe {
        Repr {
            raw: FatPtr { data, len },
        }
        .rust
    }
}

fn slice_index_order_fail(index: usize, end: usize) -> ! {
    // panic!("slice index starts at {} but ends at {}", index, end);
}

fn slice_end_index_len_fail(index: usize, len: usize) -> ! {
    // panic!("range end index {} out of range for slice of length {}", index, len);
}

unsafe impl<T> SliceIndex<[T]> for ops::Range<usize> {
    type Output = [T];

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const [T] {
        unsafe { slice_from_raw_parts(slice.as_ptr().add(self.start), self.end - self.start) }
    }

    fn index(self, slice: &[T]) -> &[T] {
        if self.start > self.end {
            slice_index_order_fail(self.start, self.end);
        } else if self.end > slice.len() {
            slice_end_index_len_fail(self.end, slice.len());
        }
        // SAFETY: `self` is checked to be valid and in bounds above.
        unsafe { &*self.get_unchecked(slice) }
    }
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = &a[1..3];
}
```
