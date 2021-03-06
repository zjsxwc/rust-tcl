# Rust Tcl
[![Build Status](https://travis-ci.org/AngryLawyer/rust-tcl.svg?branch=master)](https://travis-ci.org/AngryLawyer/rust-tcl)

Simple bindings for Tcl in Rust.

Currently targets Tcl 8.5, but support for 8.6 will be added soon.

These bindings are very new, and may take some time to stabilise.

[Documentation](http://angrylawyer.github.io/rust-tcl/tcl/)

# How to use

Make sure you have the development libraries for Tcl8.5 installed on your machine - you'll find these in Homebrew on OSX, or in your favourite package manager in your Linux distro. If you're using Windows, it may take a little more effort to set up.

Add it as a dependency into your project:

```toml
    [dependencies]
    rust-tcl = "0.3.0"
```

Or you can pull it from GitHub

```toml
    [dependencies.rust-tcl]
    git = "https://github.com/AngryLawyer/rust-tcl"
```

# Features

The current implemented feature set:

* Initializing Tcl
* Creating interpreters
* Creating simple Tcl values
* Evaluating files
* Evaluating strings
* Getting result objects and strings from an interpreter

Other features will be added as I get around to them

# Demos

A simple Tcl repl:

> cargo run --example repl

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
