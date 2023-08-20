# `reqwest` examples

These are some examples of how to use the
[reqwest](https://crates.io/crates/reqwest) library that I created while
implementing code snippet generation for the
[httpsnippet](https://github.com/Kong/httpsnippet/) repository (still
[in PR](https://github.com/Kong/httpsnippet/pull/328) currently).

You can build and run these examples by downloading the repository, and
running `cargo run` from the root directory:

```shell
git clone git@github.com:Benjscho/reqwest-examples.git
cd reqwest-examples
cargo run
```

This will run the `main.rs` file, which runs each of the fixture
examples found in `./src` in turn. 

These examples all use tokio as the async runtime. 

These examples use [mockbin](http://mockbin.com) to send 
requests to, and display the responses.
