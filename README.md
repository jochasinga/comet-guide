# comet-guide ![bare metal](assets/bare-metal-c04868.svg)

Study guide to the [OS "Comet" book][1] with code examples in Rust.

## objective

- To summarize the content of the book into a sort of study guide
- To port the C code from the book to Rust

~~The code by no mean tries to emulate the original C code. It tries to stay close when possible, but doesn't go out of the (Rust) way to map to the way C does things.~~

Well, it does go out of the way of emulating C and utilize a lot of ⚡ unsafe code ⚡. Doing so gives me the opportunity to not just learn Rust but also C and how closely it maps to OS system calls.

## roadmap

- Remove playground links (painful to maintain)
- Add safe code alternatives to those unsafe ones

[1]: http://pages.cs.wisc.edu/~remzi/OSTEP/
