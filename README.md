# wasm_extra

> Extra utilities for WASM.

So far, crate offers two things:

* the `add_event_listener_with_callback!` macro named after the `::web_sys::EventTarget::add_event_listener_with_callback()` function
and which internally uses `::wasm_bindgen::closure::Closure::new()`
to obtain JavaScript closure from Rust closure.

* the `add_event_listener_with_fn_once_callback!` macro,
which internally uses `::wasm_bindgen::closure::Closure::once_into_js()`
for handling `FnOnce` closures.

The macro is a bit more ergonomic to use and code with it is easier to mentally parse than
the corresponding code without it.

![side-by-side comparison of the macro and the function](https://i.imgur.com/YQ8QkYf.png)

In the expanded code, the event target (`open_files_btn`) appears only on line 16, whereas in the macro it's the
very first argument, which then followed by the event name, then the "closure prologue", and - lastly -
the closure itself.

"Closure prologue" is a stipulative term for the code that is executed before the closure itself. It's useful for
preparing the closure's environment, for example, by cloning the variables before capturing.

In the code above it's empty but in the following example it's not:

![example of the closure prologue](https://i.imgur.com/K3DK2vn.png)

In addition, in the code above the event target is captured by the closure. This requires special handling
and is done with the ampersand before the event target's variable.
