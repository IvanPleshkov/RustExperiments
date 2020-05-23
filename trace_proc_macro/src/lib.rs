extern crate proc_macro;

use proc_macro::*;

#[proc_macro_attribute]
#[cfg(feature = "profiling_enabled")]
pub fn trace(_: TokenStream, input: TokenStream) -> TokenStream {
    let function_name = "func";
    let trait_name = "trait";
    let struct_name = "struct";
    let string_to_insert = format!("trace_point!({}, {}, {});", function_name, trait_name, struct_name);

    let mut input = input.to_string();
    if let Some(idx) = input.find("{") {
        input.insert_str(idx+1, &string_to_insert);
    }
    input.parse().unwrap()
}

#[proc_macro_attribute]
#[cfg(not(feature = "profiling_enabled"))]
pub fn trace(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}
