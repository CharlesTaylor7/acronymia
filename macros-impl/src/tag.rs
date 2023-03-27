use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{punctuated::*, token::Comma, Data, DeriveInput, Fields, Variant};

pub fn derive(ast: DeriveInput) -> TokenStream {
    let vis = &ast.vis.clone();
    let enum_name = &ast.ident.clone();
    let tag_enum_name = format_ident!("{}Tag", enum_name);
    let variants = variants(ast);

    let pattern_suffix = variants
        .iter()
        .map(|v| match v.fields {
            Fields::Unit => quote!(),
            Fields::Unnamed(_) => quote!((_)),
            Fields::Named(_) => quote!({ .. }),
        })
        .collect::<Vec<_>>();

    let tags = variants
        .into_iter()
        .map(|v| Variant {
            fields: Fields::Unit,
            ..v
        })
        .collect::<Vec<_>>();

    quote! {
        #[derive(Clone, Debug, PartialEq)]
        #vis enum #tag_enum_name {
            #(
                #tags,
            )*
        }
        impl Tag for #enum_name {
            type Tag = #tag_enum_name;
            fn to_tag(&self) -> Self::Tag {
                match self {
                    #(
                        #enum_name::#tags #pattern_suffix => #tag_enum_name::#tags,
                    )*
                }
            }
        }
    }
}

fn variants(ast: DeriveInput) -> Punctuated<Variant, Comma> {
    if let Data::Enum(data) = ast.data {
        data.variants
    } else {
        panic!("Tag can only be derived for enums.")
    }
}
