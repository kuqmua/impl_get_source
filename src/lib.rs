#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

use proc_macro_helpers::global_variables::hardcode::ORIGIN_NAME;
use proc_macro_helpers::global_variables::hardcode::WRAPPER_NAME;

#[proc_macro_derive(ImplGetSourceFromTufaCommon)]
pub fn derive_impl_get_source_from_tufa_common(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::TufaCommon)
}

#[proc_macro_derive(ImplGetSourceFromCrate)]
pub fn derive_impl_get_source_from_crate(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::Crate)
}

fn generate(
    input: proc_macro::TokenStream,
    path: proc_macro_helpers::path::Path,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplGetSource syn::parse(input) failed");
    let ident = &ast.ident;
    let get_source_token_stream = format!("{path}::traits::get_source::GetSource")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    match ast.data {
        syn::Data::Union(_) => {
            panic!("ImplGetSource does not work on unions!")
        }
        syn::Data::Enum(data_enum) => {
            let variants = data_enum.variants.into_iter().map(|v| {
                let variant_ident = v.ident;
                let ident_as_string = variant_ident.to_string();
                let is_wrapper = if ident_as_string.contains(WRAPPER_NAME) && ident_as_string.contains(ORIGIN_NAME) {
                    panic!("ImplGetSource - ident name {} contains {} and {}", ident_as_string, WRAPPER_NAME, ORIGIN_NAME);
                }
                else if ident_as_string.contains(WRAPPER_NAME) {
                    true
                }
                else if ident_as_string.contains(ORIGIN_NAME) {
                    false
                }
                else {
                    panic!("ImplGetSource - ident name {} does not contain {} or {}", ident_as_string, WRAPPER_NAME, ORIGIN_NAME);
                };
                match v.fields {
                    syn::Fields::Unit => panic!(
                        "ImplGetSource still not work with syn::Fields::Unit"
                    ),
                    syn::Fields::Named(fields_named) => {
                        let fields_list = fields_named.named.iter().map(|field_named| {
                            let field_ident = &field_named.ident;
                            quote::quote! {
                                #field_ident
                            }
                        });
                        let fields_logic = fields_named.named.iter().map(|field_named| {
                            let field_ident = &field_named.ident;
                                match &field_named.ty {
                                    syn::Type::Path(type_path) => {
                                        let segment_ident = format!("{}", type_path.path.segments[0].ident);
                                        let (
                                            origin_or_wrapper_for_vec, 
                                            origin_or_wrapper_for_hashmap, 
                                            origin_or_wrapper_simple
                                        ) = match is_wrapper {
                                            true => (
                                                quote::quote! {error.get_source()}, 
                                                quote::quote! {error.get_source()}, 
                                                quote::quote! {#field_ident.get_source()}
                                            ),
                                            false => (
                                                quote::quote! {error}, 
                                                quote::quote! {error}, 
                                                quote::quote! {#field_ident}
                                            ),
                                        };
                                        if segment_ident == *"Vec" {
                                            quote::quote! {
                                                let mut #field_ident = #field_ident
                                                    .iter()
                                                    .map(|error| format!("{} ,", #origin_or_wrapper_for_vec))
                                                    .fold(String::from(""), |mut acc, elem| {
                                                        acc.push_str(&elem);
                                                        acc
                                                    }
                                                );
                                                if !#field_ident.is_empty() {
                                                    #field_ident.pop();
                                                    #field_ident.pop();
                                                }
                                                #field_ident = format!("[{}]", #field_ident);
                                            }
                                        }
                                        else if segment_ident == *"HashMap" {
                                            quote::quote! {
                                                let mut #field_ident = #field_ident
                                                    .iter()
                                                    .map(|(key, error)| format!("[{} {}],", key, #origin_or_wrapper_for_hashmap))
                                                    .fold(String::from(""), |mut acc, elem| {
                                                        acc.push_str(&elem);
                                                        acc
                                                    }
                                                );
                                                if !#field_ident.is_empty() {
                                                    #field_ident.pop();
                                                }
                                                let #field_ident = format!("[{}]", #field_ident);
                                            }
                                        }
                                        else {
                                            quote::quote! {
                                                let #field_ident = #origin_or_wrapper_simple;
                                            }
                                        }
                                    },
                                    _ => panic!("ImplGetSource only work on enums with Path(type_path)!")
                                }
                        });
                        let mut fields_bracket_string = fields_named.named.iter().map(|_| {
                            String::from("{} ")
                        })
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&elem);
                            acc
                        });
                        if !fields_bracket_string.is_empty() {
                            fields_bracket_string.pop();
                        }
                        fields_bracket_string = format!("[{}]", fields_bracket_string);
                        let fields_arguments = fields_named.named.iter().map(|field_named| {
                            let field_ident = &field_named.ident;
                            quote::quote! {
                                #field_ident
                            }
                        });
                        quote::quote! {
                             #ident::#variant_ident{
                                #(#fields_list,)*
                             } => {
                                #(#fields_logic)*
                                format!(#fields_bracket_string, #(#fields_arguments,)*)
                             }
                        }
                    }
                    syn::Fields::Unnamed(unnamed) => {
                        match unnamed.unnamed.len() {
                            1 => {
                                match &unnamed.unnamed[0].ty {
                                    syn::Type::Path(type_path) => {
                                        let segment_ident = format!("{}", type_path.path.segments[0].ident);
                                        let (
                                            origin_or_wrapper_for_vec, 
                                            origin_or_wrapper_for_hashmap, 
                                            origin_or_wrapper_simple,
                                        ) = match is_wrapper {
                                            true => (
                                                quote::quote! {error.get_source()}, 
                                                quote::quote! {error.get_source()}, 
                                                quote::quote! {error.get_source()}
                                            ),
                                            false => (
                                                quote::quote! {format!("{}, ", error)},
                                                quote::quote! {error},
                                                quote::quote! {format!("{}", error)}
                                            ),
                                        };
                                        if segment_ident == *"Vec" {
                                            quote::quote! {
                                                #ident::#variant_ident(error_vec) => {
                                                    let mut formatted = error_vec
                                                        .iter()
                                                        .map(|error| #origin_or_wrapper_for_vec)
                                                        .fold(String::from(""), |mut acc, elem| {
                                                                acc.push_str(&elem);
                                                                acc
                                                        }
                                                    );
                                                    if !formatted.is_empty() {
                                                        formatted.pop();
                                                        formatted.pop();
                                                    }
                                                    format!("[{}]", formatted)
                                                }
                                            }
                                        }
                                        else if segment_ident == *"HashMap" {
                                            quote::quote! {
                                                #ident::#variant_ident(error_hashmap) => {
                                                    let mut formatted = error_hashmap
                                                        .iter()
                                                        .map(|(key, error)| format!("[{} {}],", key, #origin_or_wrapper_for_hashmap))
                                                        .fold(String::from(""), |mut acc, elem| {
                                                                acc.push_str(&elem);
                                                                acc
                                                        }
                                                    );
                                                    if !formatted.is_empty() {
                                                        formatted.pop();
                                                    }
                                                    format!("[{}]", formatted)
                                                }
                                            }
                                        }
                                        else {
                                            quote::quote! {
                                                #ident::#variant_ident(error) => #origin_or_wrapper_simple
                                            }
                                        }
                                    },
                                    _ => panic!("ImplGetSource only work on enums with Path(type_path)!")
                                }
                            }
                            _ => panic!("ImplGetSource only work on enums with unnamed.len() == 1!")
                        }
                    }
                }
            });
            let gen = quote::quote! {
                impl #get_source_token_stream for #ident {
                    fn get_source(&self) -> String {
                        match self {
                            #(#variants,)*
                        }
                    }
                }
            };
            gen.into()
        }
        syn::Data::Struct(data_struct) => {
            let ident_as_string = ident.to_string();
            let is_wrapper = if ident_as_string.contains(WRAPPER_NAME) && ident_as_string.contains(ORIGIN_NAME) {
                panic!("ImplGetSource - ident name {} contains {} and {}", ident_as_string, WRAPPER_NAME, ORIGIN_NAME);
            }
            else if ident_as_string.contains(WRAPPER_NAME) {
                true
            }
            else if ident_as_string.contains(ORIGIN_NAME) {
                false
            }
            else {
                panic!("ImplGetSource - ident name {} does not contain {} or {}", ident_as_string, WRAPPER_NAME, ORIGIN_NAME);
            };
            match data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    match fields_named.named.len() {
                        2 => {
                            let source_field_ident = fields_named.named[0]
                                .ident
                                .clone()
                                .expect("ImplGetSource - there is no first field ident!");
                            if format!("{}", source_field_ident) != *"source" {
                                panic!("ImplGetSource - no 'source'-named field found!");
                            }
                            match fields_named.named[0].ty.clone() {
                                syn::Type::Path(type_path) => {
                                    let possible_vec_or_hashmap_ident_as_string =
                                        format!("{}", type_path.path.segments[0].ident);
                                    let (
                                        origin_or_wrapper_for_vec, 
                                        origin_or_wrapper_for_hashmap, 
                                        origin_or_wrapper_simple,
                                    ) = match is_wrapper {
                                        true => (
                                            quote::quote! {error.get_source()}, 
                                            quote::quote! {error.get_source()}, 
                                            quote::quote! {self.source.get_source()}
                                        ),
                                        false => (
                                            quote::quote! {format!("{}, ", error)},
                                            quote::quote! {error},
                                            quote::quote! {format!("{}", self.source)}
                                        ),
                                    };
                                    let gen = if possible_vec_or_hashmap_ident_as_string == *"Vec" {
                                        quote::quote! {
                                            let mut formatted = self
                                            .source
                                            .iter()
                                            .map(|error| format!("{},", #origin_or_wrapper_for_vec))
                                            .fold(String::from(""), |mut acc, elem| {
                                                acc.push_str(&elem);
                                                acc
                                            });
                                            if !formatted.is_empty() {
                                                formatted.pop();
                                                formatted.pop();
                                            }
                                            formatted
                                        }
                                    } else if possible_vec_or_hashmap_ident_as_string == *"HashMap"
                                    {
                                        quote::quote! {
                                            let mut formatted = self
                                            .source
                                            .iter()
                                            .map(|(key, error)| format!("{} {},", key, #origin_or_wrapper_for_hashmap))
                                            .collect::<Vec<String>>()
                                            .iter()
                                            .fold(String::from(""), |mut acc, elem| {
                                                acc.push_str(elem);
                                                acc
                                            });
                                            if !formatted.is_empty() {
                                                formatted.pop();
                                            }
                                            formatted
                                        }
                                    } else {
                                        quote::quote! {
                                            #origin_or_wrapper_simple
                                        }
                                    };
                                    let generated = quote::quote! {
                                        impl #get_source_token_stream for #ident {
                                            fn get_source(&self) -> String {
                                                #gen
                                            }
                                        }
                                    };
                                    generated.into()
                                }
                                _ => panic!("ImplGetSource only work on Type::Path!"),
                            }
                        }
                        _ => panic!(
                            "ImplGetSource only work on structs with 2 named fields!"
                        ),
                    }
                }
                _ => panic!("ImplGetSource only work with syn::Fields::Named!"),
            }
        }
    }
}
