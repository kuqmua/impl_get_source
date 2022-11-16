#[proc_macro_derive(ProcMacroExampleName)]
pub fn derive_(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ProcMacroExampleName syn::parse(input) failed");
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Union(_) => panic!("ProcMacroExampleName does not work on union!"),
        syn::Data::Struct(_) => panic!("ProcMacroExampleName does not work on structs!"),
        syn::Data::Enum(_) => panic!("ProcMacroExampleName does not work on enums!"),
    }
    let gen = quote::quote! {};
    gen.into()
}
