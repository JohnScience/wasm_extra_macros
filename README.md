# wasm_extra_macros

[![Latest Version](https://img.shields.io/crates/v/wasm_extra_macros.svg)][`wasm_extra_macros`]
[![Downloads](https://img.shields.io/crates/d/wasm_extra_macros.svg)][`wasm_extra_macros`]
[![Documentation](https://docs.rs/wasm_extra_macros/badge.svg)][`wasm_extra_macros`/docs]
[![License](https://img.shields.io/crates/l/wasm_extra_macros.svg)][`wasm_extra_macros`/license]
[![Dependency Status](https://deps.rs/repo/github/JohnScience/wasm_extra_macros/status.svg)][`wasm_extra_macros`/dep_status]

> Macros for [`wasm_extra`].

So far, crate offers two things:

* the `add_event_listener_with_callback!` macro named after the
`::web_sys::EventTarget::add_event_listener_with_callback()` function
and which internally uses `::wasm_bindgen::closure::Closure::new()`
to obtain JavaScript closure from Rust closure.

* the `add_event_listener_with_fn_once_callback!` macro,
which internally uses `::wasm_bindgen::closure::Closure::once_into_js()`
for handling `FnOnce` closures.

The macro is a bit more ergonomic to use and code with it is easier to mentally
parse than the corresponding code without it.

![side-by-side comparison of the macro and the function](https://i.imgur.com/YQ8QkYf.png)

In the expanded code, the event target (`open_files_btn`) appears only on line
16, whereas in the macro it's the very first argument, which then followed by
the event name, then the "closure prologue", and - lastly - the closure itself.

"Closure prologue" is a stipulative term for the code that is executed before
the closure itself. It's useful for preparing the closure's environment, for
example, by cloning the variables before capturing.

In the code above it's empty but in the following example it's not:

![example of the closure prologue](https://i.imgur.com/K3DK2vn.png)

In addition, in the code above the event target is captured by the closure.
This requires special handling and is done with the ampersand before the
event target's variable.

## SemVer Policy

At the moment, there's no any semver guarantees. The crate is being inactively developed.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`wasm_extra`]: https://crates.io/crates/wasm_extra
[`wasm_extra_macros`]: https://crates.io/crates/wasm_extra_macros
[`wasm_extra_macros`/docs]: https://docs.rs/wasm_extra_macros
[`wasm_extra_macros`/license]: https://github.com/JohnScience/wasm_extra_macros#license
[`wasm_extra_macros`/dep_status]: https://deps.rs/repo/github/JohnScience/wasm_extra_macros
