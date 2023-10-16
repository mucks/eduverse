use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn wasm_account(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote! {
        #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
        #input
    };
    output.into()
}
