use proc_macro::TokenStream;
use syn::DeriveInput;

fn impl_refelective_macro(ast: DeriveInput)-> TokenStream {
    let struct_ident = ast.ident;
    let struct_ident_str = struct_ident.to_string();

    quote::quote!({
        impl Reflective for #struct_ident{
            fn name(&self) -> &'static str {
                #struct_ident_str
            }
        }
    })
    .into()
}

#[proc_macro_derive(Reflective)]
pub fn refelective_derive_macro (item : TokenStream) -> TokenStream {
    let ast : DeriveInput = syn::parse(item).unwrap();

    impl_refelective_macro(ast)
}