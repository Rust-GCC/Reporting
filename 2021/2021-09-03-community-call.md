# GCC Rust Meeting 2021-09-03

- Date/Time: 3rd Sept 2021 10h00 BST
- Mailing list: https://gcc.gnu.org/mailman/listinfo/gcc-rust
- Zulip: https://gcc-rust.zulipchat.com/
- IRC: irc.oftc.net #gccrust
- Video Link: https://meet.jit.si/259057065581073

## TLDR

This was a well attended meeting for those interested in GCC Rust. We are seeking feedback on interest for a 2nd community call to suit those in the US timezone, we wish to keep this meeting the main one as most of the community are european timezones. The Project is slightly behind its planned estimates but the community has been pushing forward in future area's of work so this will save time again in the future. The coding standards of gccrs are in question as we have used some mixed styles of C++11 in the code base so we will work on figuring out what our coding conventions are/will be. Sticking to c++11 is important in order to be merge-able upsteam with GCC. The meeting spent alot of time discussing the compiler options question with regards to strict-aliasing please read that section for context.

## Agenda

* Project Status
* GCC Rust on Tour
    * BCS Rust London 2021
    * LPC 2021
* Good First Issues
* Strict Aliasing GitHub Issue
* Questions

## Project Status

Monthly Report for August 2021: https://github.com/Rust-GCC/Reporting/blob/main/2021-08-monthly-report.org

https://docs.google.com/spreadsheets/d/1B_JFzHgGclpdtPcQvnThkNJnP7Hh8fCIAU1rYFu_23M/edit?usp=sharing

Slightly behind where Philip wishes to be but this was always to be expected, it was difficult to map out estimates for the milestones.

### Thanks to the community

Unions, modules and multiple file parsing all being progressed by the community. This will help Philip make up for lost time in Traits

## GCC Rust on Tour

### BCS Rust London 2021

See the video here: https://www.youtube.com/watch?v=kdsYi1E0cEQ

### LPC 2021

I will be giving a talk about GCC Rust on the 20th September 2021 at 0900 (us-pacific-time). Find more information over on https://linuxplumbersconf.org/event/11/contributions/911/

Thanks to Miguel Ojeda for reaching out to let me know about the Rust toolchain within the Kernel microconference: https://linuxplumbersconf.org/event/11/contributions/970/

Registration for all of LPC:
<https://www.linuxplumbersconf.org/event/11/page/112-attend>

### Kangrejos Rust Linux Workshop

[Arthur Cohen](https://github.com/CohenArthur/) will be speaking at this workshop https://kangrejos.com/ which might be of interest and link up to some discussion on the Rust toolchain and Linux.

## Good First Issues

Lints, HIR desugaring, feature work? What issues are people interested in working on? is there anything we ca do to help getting started easier?

[Jeremy Bennett](https://github.com/jeremybennett) suggested keeping the c++11 usage to a limited subset helps include contributors from GCC world and others as the latest modern C++ features can be hard to navigate in big code bases. Mark Wieelard also aggreed here and pointed out the usage of c++11 lambda's that we are removing which was a mistake by Philip early on when trying to figure out the best way to work with the IR.

Lets create some issues around codeing conventions and actually define it. We are aleady using a clang-format automation can this be extended to look for c++ features? Does out code style map to GCC's coding standards?

At the BCS conference the time constraints meant one question about getting involved with GCC Rust was dropped, Philip will draft a blog post to address this for the BCS blog.

## Strict Aliasing GitHub Issue

https://github.com/Rust-GCC/gccrs/issues/653

I am seeking feedback from the community about this issue. Especially around the usage of common compiler flags. 

[Thomas Schwinge](https://github.com/tschwinge) suggested what do we gain by allowing people to change the alising rules if it will cause issues? I think this is a valid point I can see in other front-ends this paticular option does get hard set: https://github.com/Rust-GCC/gccrs/blob/master/gcc/go/go-backend.c#L84-L92

[Philipp Krones](https://github.com/flip1995) suggested that we should be careful to not allow those to bypass the Rust RFC processes for new language features and suggested the following FAQ update.

[Philip Herron](https://github.com/philberty) believes it seems wierd going out of our way to actively disable compiler options and we should allow people to choose. Though we do not wish for people to design new language features outside of the Rustc processes. We may wish to have our own lints and not limit ourselves in the future to have to seek permission to innovate.

### Changing the FAQ

https://gcc.gnu.org/pipermail/gcc-rust/2021-September/000171.html

```
## What is the plan for inconsistencies in behaviour?

**If gccrs interprets a program differently from rustc, this is considered a bug.**

Once Rust-GCC can compile and verify all Rust programs, this can also help figure out any inconsistencies in the specification of features in the language. This should help to get features right in _both_ compilers before they are stabilized.

The GCC Rust project is not and will not provide a shortcut for getting features into the Rust language. It will follow the well established processes, i.e. RFCs.
```

I believe most of us are in favour of this FAQ update but lets take our time before actually putting it up there to be sure we are happy with this. I do not wish to sign up to a policy to end up breaking it down the line.

## Questions

### This meetings date/time

I am seeking feedback about this meetings date/time. Do we want to move it to the evening to allow those interested in America to join?

This meeting time is good for those who already join, lets keep it this way for now. Philip will investigate if there are those in the US timezeone wishing to join and hold a seperate one for this.
