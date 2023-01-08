use proc_macro::TokenStream;
use syn::parse_macro_input;

mod args;

use args::{FnMutArgs, FnOnceArgs};

#[proc_macro]
pub fn add_event_listener_with_callback(ts: TokenStream) -> TokenStream {
    let args = parse_macro_input!(ts as FnMutArgs);
    args.handle().into()
}

#[proc_macro]
pub fn add_event_listener_with_fn_once_callback(ts: TokenStream) -> TokenStream {
    let args = parse_macro_input!(ts as FnOnceArgs);
    args.handle().into()
}
