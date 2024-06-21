use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn debug_masked_derive_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("DebugMasked can only be used with structs"),
    };

    let field_names = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let field_values = fields.iter().map(|f| {
        let ident = &f.ident;
        let mut is_masked = false;
        for attr in &f.attrs {
            if attr.path().is_ident("masked") {
                is_masked = true;
                break;
            }
        }
        if is_masked {
            quote! { "*****".to_string() }
        } else {
            quote! { format!("{:?}", self.#ident) }
        }
    });

    let gen = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut parts = vec![];
                #(
                    parts.push(format!("{}: {}", stringify!(#field_names), #field_values));
                )*
                if f.alternate() {
                    write!(f, "{} {{\n    {}\n}}", stringify!(#name), parts.join(",\n    "))
                } else {
                    write!(f, "{} {{ {} }}", stringify!(#name), parts.join(", "))
                }
            }
        }
    };
    gen.into()
}
