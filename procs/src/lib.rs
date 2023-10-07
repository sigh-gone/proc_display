extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};


#[proc_macro]
pub fn display_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => return quote! {}.into(),
        },
        _ => return quote! {}.into(),
    };

    let field_value_pairs = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap(); // Field name
        let field_value = quote! { &self.#field_name }; // Field value
        quote! {
            println!("{} - {:?}", stringify!(#field_name), #field_value);
        }
    });

    let expanded = quote! {
        impl #struct_name {
            fn display_fields(&self) {
                #( #field_value_pairs )*
            }
        }
    };

    expanded.into()
}

