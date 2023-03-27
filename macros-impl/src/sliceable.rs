use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{punctuated::*, token::Comma, Fields::*, Type, *};

pub fn derive(ast: DeriveInput) -> TokenStream {
    let vis = &ast.vis;
    let struct_name = &ast.ident;
    let sliced_struct_name = format_ident!("{}_Sliced", struct_name);
    let fields = fields(&ast);
    let sliced_fields_def = fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            Field {
                ty: Type::Verbatim(quote! {
                    ::macros::slice::JoinedSignal<#ty>
                }),
                ..f.clone()
            }
        })
        .collect::<Punctuated<Field, Comma>>();

    let field_names = fields
        .into_iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<&Ident>>();

    quote! {
        #[allow(non_camel_case_types)]
        #vis struct #sliced_struct_name {
            #sliced_fields_def
        }
        impl Sliceable for #struct_name {
            type Sliced = #sliced_struct_name;
            fn slice(signal: RwSignal<Self>, cx: Scope) -> Self::Sliced {
                #sliced_struct_name {
                    #(
                        #field_names: ::macros::slice::__create_slice(
                            cx,
                            signal,
                            move|state| state.#field_names.clone(),
                            move|state, value| state.#field_names = value,
                        ),
                    )*
                }
            }
        }
    }
}

fn fields(ast: &DeriveInput) -> &Punctuated<Field, Comma> {
    let data = if let Data::Struct(data) = &ast.data {
        data
    } else {
        panic!("Sliceable can only be derived for structs with named fields .")
    };

    if let Named(FieldsNamed { named, .. }) = &data.fields {
        &named
    } else {
        panic!("Sliceable can only be derived for structs with named fields .")
    }
}
