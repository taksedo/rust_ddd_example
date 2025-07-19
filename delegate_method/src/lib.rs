use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_attribute]
pub fn delegate_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = attr.to_string().trim().to_string();
    let field_name = Ident::new(&attr_str, proc_macro2::Span::call_site());

    let item_ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &item_ast.ident;

    let syn::Data::Struct(data_struct) = &item_ast.data else {
        panic!("Only structs are supported");
    };

    let field_ty = data_struct
        .fields
        .iter()
        .find(|f| f.ident.as_ref() == Some(&field_name))
        .map(|f| &f.ty)
        .expect("Field not found");

    let generics = if let syn::Type::Path(type_path) = field_ty {
        let path = &type_path.path;
        let last = path.segments.last().unwrap();
        let args = &last.arguments;

        if let syn::PathArguments::AngleBracketed(angle_args) = args {
            angle_args
                .args
                .iter()
                .filter_map(|arg| {
                    if let syn::GenericArgument::Type(ty) = arg {
                        Some(ty.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        } else {
            panic!("Expected generics on field");
        }
    } else {
        panic!("Unsupported field type");
    };

    if generics.is_empty() {
        panic!("No generics found in field type");
    }

    let cart_id_type = &generics[0];
    let version_type = quote! { Version };

    let methods = vec![
        (
            Ident::new("id", proc_macro2::Span::call_site()),
            quote! { #cart_id_type },
        ),
        (
            Ident::new("version", proc_macro2::Span::call_site()),
            version_type,
        ),
    ];

    let methods = methods.into_iter().map(|(method_name, ty)| {
        quote! {
            pub fn #method_name(&self) -> &#ty {
                &self.#field_name.#method_name()
            }
        }
    });

    let output = quote! {
        #item_ast

        impl #struct_name {
            #(#methods)*
        }
    };

    output.into()
}
