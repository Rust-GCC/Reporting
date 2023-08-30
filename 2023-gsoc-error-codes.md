# `Adding Rustc Error Codes to gccrs`

- Name: Muhammad Mahad
- Github Profile URL: https://github.com/MahadMuhammad

## Overview

#### What is the project? 
The project aimed to enhance the user experience of the GNU GCC Rust (GCCRS) frontend by implementing error codes compatible with Rust's compiler (rustc). The primary objective was to enable the integration of test suites from both compilers, allowing the Rust testsuite to be executed on the GCCRS compiler.

The key focus was on improving error reporting and user guidance. This was done by introducing informative error codes and messages, along with rich location information that assists both users and developers in understanding and rectifying code issues. The project also encompassed conducting a thorough comparison between the test suites of Rust's compiler and GCCRS. This comparison process revealed various issues, including bugs and internal compiler errors within GCCRS.

#### What are rustc Error codes, show an example? 
Rustc error codes are unique identifiers assigned to specific common errors encountered in Rust programs. These error codes serve as a way to precisely identify and communicate the nature of an error in the codebase. They offer several features that contribute to a more effective debugging and troubleshooting process:

- **Unique Identification**: Each error code is a distinct identifier associated with a particular type of error. This makes it easier for developers to understand the issue at hand and look up relevant information.

- **Hyperlinked Documentation**: Rustc error codes are hyperlinked to detailed documentation that provides comprehensive explanations of the error, its potential causes, and suggestions for resolving it.

- **Examples and Fixes**: The documentation for each error code often includes illustrative code examples that trigger the error, as well as recommended solutions for addressing it. This helps developers comprehend the context of the error and how to rectify it.

- **Helpful Error Messages**: In addition to error codes, rustc also emits helpful error messages that provide context and insights into the nature of the error. These messages offer guidance on what specifically went wrong and how to address it.

```rust
fn f(u: i32) {}

f(); // error: invalid number of arguments 
```
When this code is compiled using rustc, it will generate the error code E0061. The error message associated with E0061 would be:

```rust
error[E0061]: this function takes 1 argument but 0 arguments were supplied
 --> src/main.rs:5:1
  |
5 | f(); // error!
  | ^-- an argument of type `i32` is missing
  |
note: function defined here
 --> src/main.rs:3:4
  |
3 | fn f(u: i32) {}
  |    ^ ------
```
By combining error codes, hyperlinked documentation, explanatory messages, and examples, rustc helps developers quickly understand and address errors in their Rust code.


#### Why are error codes useful:

**Consistency and Compatibility**: Emitting error codes that align with those produced by rustc creates a bridge of compatibility between the test suites of rustc and gccrs. This ensures that both compilers are evaluated using the same set of error cases, enabling direct comparison and identification of discrepancies.

**Bug Detection and Resolution**: The alignment of error codes facilitates the detection of bugs and inconsistencies within GCCRS. When gccrs generates error codes that differ from rustc for the same input, it signals the presence of potential issues in the GCCRS compiler's error handling logic. Addressing these discrepancies becomes a key step in improving the overall stability and correctness of GCCRS.

**User Experience Improvement**: The use of error codes to improve the compatibility between gccrs and rustc also has a direct positive impact on the user experience. Developers using gccrs will benefit from informative error messages and detailed locations that guide them in understanding and rectifying code issues. This enriches the overall development experience and accelerates the debugging process.

#### Challenges to adding Error Codes?
- **Ambiguity in Error Codes**: While integrating error codes, there were instances where a single code point in GCCRS could potentially trigger multiple error codes. This introduced ambiguity in determining the most appropriate error code to emit. To resolve this, I had to modify the codebase and implement additional checks to ensure that the correct error code was consistently emitted, aligning with rustc.
- **Consistency with rustc**: For making gccrs error codes compatible with rustc, we conducted a comprehensive comparison between the rustc test suite and gccrs, and rustc can be found on this [comparison page](https://gist.github.com/MahadMuhammad/8c9d5fc88ea18d8c520937a8071d4185#file-gccrs-errorcode-support-md). 
  -  While this comparison, we encountered several issues, including bugs and internal compiler errors. This iterative process led to substantial improvements in the gccrs stability.
- While adding some error codes, we encountered some code points where more than one error code can be emitted. 
  -  This required us to modify the code and add some additional checks to make sure that the error code emitted is the same as rustc.

#### What difficulties did you face and what did you learn?
- The most significant skill I acquired involved navigating extensive C++ codebases. It was my inaugural experience with such immense code complexity. 
- Also, adding and testing this project manually requires a careful obersevation of the code and the error messages. This led me to use project management tools like [GitHub Project](https://github.com/users/MahadMuhammad/projects/7/views/1)
- Also, there are some errorcodes, which are no longer emitted by rustc-1.49.0. 
  - Therefore, I utilized [Python-driven](https://gist.github.com/MahadMuhammad/8c9d5fc88ea18d8c520937a8071d4185#file-rustc-errorcode-parser-py) automation to validate error codes and messages. This approach streamlined the process, obviating manual effort and minimizing error risks.
- Embraced a paradigm shift: Prioritizing progress over perfection. A sage piece of advice from my mentor, [Philip Herron](https://github.com/philberty/), catalyzed my work approach.

### Example error messages


In a prior development phase, [David Malcolm](https://github.com/davidmalcolm) extended the capabilities of GCC diagnostics to incorporate support for associating diagnostics with rules from coding standards. We are using this feature to associate gcc-rust error codes. The initial introduction included a solitary error, denoted as [E0054](https://doc.rust-lang.org/error_codes/E0054.html) , addressing non-allowed cast operations to bool. Following the integration of further type-checking mechanisms, gccrs commenced emitting error codes and messages that closely resembled those of rustc.

```rust
fn main() {
    let x = 5;
    let x_is_nonzero = x as bool; 

    0u32 as char; 

    let x = &[1_usize, 2] as [usize];

    let a = &0u8; 
    let y: u32 = a as u32; 
}
```
### Previous State:
The previous version exhibited the following output:
```bash
~/gccrs/gcc/testsuite/rust/compile/all-cast.rs:3:24: error: invalid cast ‘<integer>’ to ‘bool’ [E0054]
    3 |     let x_is_nonzero = x as bool; 
      |                        ^    ~~~~
~/gccrs/gcc/testsuite/rust/compile/all-cast.rs:5:5: error:invalid cast ‘u32’ to ‘char’ [E0054]
    5 |     0u32 as char; 
      |     ^~~~    ~~~~
~/gccrs/gcc/testsuite/rust/compile/all-cast.rs:7:13: error: invalid cast ‘& [usize:CAPACITY]’ to ‘[usize]’ [E0054]
    7 |     let x = &[1_usize, 2] as [usize]; 
      |             ^                ~
~/gccrs/gcc/testsuite/rust/compile/all-cast.rs:10:18: error: invalid cast ‘& u8’ to ‘u32’ [E0054]
   10 |     let y: u32 = a as u32;
      |                  ^    ~~~
```
### Current State:
With the incorporation of the latest type checking code, the system now emits more specific error codes and associated error messages.

```bash
~/gccrs/gcc/testsuite/rust/compile/all-cast.rs:3:24: error: cannot cast ‘<integer>’ as ‘bool’ [E0054]
    3 |     let x_is_nonzero = x as bool;
      |                        ^    ~~~~
~/gccrs/gcc/testsuite/rust/compile/all-cast.rs:5:5: error: cannot cast ‘u32’ as ‘char’, only ‘u8’ can be cast as ‘char’ [E0604]
    5 |     0u32 as char; 
      |     ^~~~    ~~~~
~/gccrs/gcc/testsuite/rust/compile/all-cast.rs:7:13: error: cast to unsized type: ‘& [usize:CAPACITY]’ as ‘[usize]’ [E0620]
    7 |     let x = &[1_usize, 2] as [usize]; 
      |             ^                ~
~/gccrs/gcc/testsuite/rust/compile/all-cast.rs:10:18: error: casting ‘& u8’ as ‘u32’ is invalid [E0606]
   10 |     let y: u32 = a as u32; 
      |                  ^    ~~~
```

### Error codes added

Add your list of your error codes

1. [`E0015`](https://doc.rust-lang.org/error_codes/E0015.html) **Error Description:** A non-`const` function was called in a `const` context.
1. [`E0023`](https://doc.rust-lang.org/error_codes/E0023.html) **Error Description:** A pattern attempted to extract an incorrect number of fields from a variant.
1. [`E0026`](https://doc.rust-lang.org/error_codes/E0026.html) **Error Description:** A struct pattern attempted to extract a nonexistent field from a struct.
1. [`E0027`](https://doc.rust-lang.org/error_codes/E0027.html) **Error Description:** A pattern for a struct fails to specify a sub-pattern for every one of the struct's fields.
1. [`E0034`](https://doc.rust-lang.org/error_codes/E0034.html)  **Error Description:** The compiler doesn't know what method to call because more than one method
1.he same prototype.
1. [`E0045`](https://doc.rust-lang.org/error_codes/E0045.html) **Error Description:** Variadic parameters have been used on a non-C ABI function.
1. [`E0046`](https://doc.rust-lang.org/error_codes/E0046.html) **Error Description:** Items are missing in a trait implementation.
1. [`E0053`](https://doc.rust-lang.org/error_codes/E0053.html) **Error Description:** The parameters of any trait method must match between a trait implementation and the trait definition.
1. [`E0054`](https://doc.rust-lang.org/error_codes/E0054.html) **Error Description:** It is not allowed to cast to a bool.
1. [`E0061`](https://doc.rust-lang.org/error_codes/E0061.html) **Error Description:** An invalid number of arguments was passed when calling a function.
1. [`E0063`](https://doc.rust-lang.org/error_codes/E0063.html) **Error Description:** A struct's or struct-like enum variant's field was not provided.
1. [`E0070`](https://doc.rust-lang.org/error_codes/E0070.html) **Error Description:** An assignment operator was used on a non-place expression.
1. [`E0093`](https://doc.rust-lang.org/error_codes/E0093.html) **Error Description:** An unknown intrinsic function was declared.
1. [`E0107`](https://doc.rust-lang.org/error_codes/E0107.html) **Error Description:** An incorrect number of generic arguments was provided.
1. [`E0124`](https://doc.rust-lang.org/error_codes/E0124.html) **Error Description:** A struct was declared with two fields having the same name.
1. [`E0133`](https://doc.rust-lang.org/error_codes/E0133.html) **Error Description:** Unsafe code was used outside of an unsafe block.
1. [`E0164`](https://doc.rust-lang.org/error_codes/E0164.html) **Error Description:** Something which is neither a tuple struct nor a tuple variant was used as a pattern.
1. [`E0229`](https://doc.rust-lang.org/error_codes/E0229.html) **Error Description:** An associated type binding was done outside of the type parameter declaration and `where` clause.
1. [`E0268`](https://doc.rust-lang.org/error_codes/E0268.html)  **Error Description:** A loop keyword (`break` or `continue`) was used outside of a loop.
1. [`E0271`](https://doc.rust-lang.org/error_codes/E0271.html)  **Error Description:** A type mismatched an associated type of a trait.
1. [`E0277`](https://doc.rust-lang.org/error_codes/E0277.html) **Error Description:** You tried to use a type which doesn't implement some trait in a place which expected that trait.
1. [`E0282`](https://doc.rust-lang.org/error_codes/E0282.html) **Error Description:** The compiler could not infer a type and asked for a type annotation.
1. [`E0308`](https://doc.rust-lang.org/error_codes/E0308.html) **Error Description:** Expected type did not match the received type.
1. [`E0323`](https://doc.rust-lang.org/error_codes/E0323.html) **Error Description:** An associated const was implemented when another trait item was expected.
1. [`E0380`](https://doc.rust-lang.org/error_codes/E0380.html) **Error Description:** An auto trait was declared with a method or an associated item.
1. [`E0391`](https://doc.rust-lang.org/error_codes/E0391.html) **Error Description:** A type dependency cycle has been encountered.
1. [`E0412`](https://doc.rust-lang.org/error_codes/E0412.html) **Error Description:** A used type name is not in scope.
1. [`E0423`](https://doc.rust-lang.org/error_codes/E0423.html) **Error Description:** An identifier was used like a function name or a value was expected and the identifier exists but it belongs to a different namespace.
1. [`E0425`](https://doc.rust-lang.org/error_codes/E0425.html) **Error Description:** An unresolved name was used.
1. [`E0426`](https://doc.rust-lang.org/error_codes/E0426.html) **Error Description:** An undeclared label was used.
1. [`E0433`](https://doc.rust-lang.org/error_codes/E0433.html) **Error Description:** An undeclared crate, module, or type was used.
1. [`E0532`](https://doc.rust-lang.org/error_codes/E0532.html) **Error Description:** Pattern arm did not match expected kind.
1. [`E0541`](https://doc.rust-lang.org/error_codes/E0541.html) **Error Description:** An unknown meta item was used.
1. [`E0571`](https://doc.rust-lang.org/error_codes/E0571.html) **Error Description:** A `break` statement with an argument appeared in a non-`loop` loop.
1. [`E0572`](https://doc.rust-lang.org/error_codes/E0572.html) **Error Description:** A return statement was found outside of a function body.
1. [`E0573`](https://doc.rust-lang.org/error_codes/E0573.html) **Error Description:** Something other than a type has been used when one was expected.
1. [`E0592`](https://doc.rust-lang.org/error_codes/E0592.html) **Error Description:** This error occurs when you defined methods or associated functions with same name.
1. [`E0599`](https://doc.rust-lang.org/error_codes/E0599.html) **Error Description:** This error occurs when a method is used on a type which doesn't implement it:
1. [`E0604`](https://doc.rust-lang.org/error_codes/E0604.html)  **Error Description:** A cast to `char` was attempted on a type other than `u8`.
1. [`E0606`](https://doc.rust-lang.org/error_codes/E0606.html) **Error Description:** An incompatible cast was attempted.
1. [`E0620`](https://doc.rust-lang.org/error_codes/E0620.html) **Error Description:** A cast to an unsized type was attempted.
1. [`E0635`](https://doc.rust-lang.org/error_codes/E0635.html) **Error Description:** The `#![feature]` attribute specified an unknown feature.
1. [`E0658`](https://doc.rust-lang.org/error_codes/E0658.html) **Error Description:** An unstable feature was used.
1. [`E0703`](https://doc.rust-lang.org/error_codes/E0703.html) **Error Description:** Invalid ABI (Application Binary Interface) used in the code.
1. [`E0753`](https://doc.rust-lang.org/error_codes/E0753.html) **Error Description:** An inner doc comment was used in an invalid context.
1. [`E0754`](https://doc.rust-lang.org/error_codes/E0754.html) **Error Description:** A non-ASCII identifier was used in an invalid context.
1. [`E0769`](https://doc.rust-lang.org/error_codes/E0769.html) **Error Description:** A tuple struct or tuple variant was used in a pattern as if it were a struct or struct variant.
1. [`E0603`](https://doc.rust-lang.org/error_codes/E0603.html) **Error Description:** A private item was used outside its scope.
