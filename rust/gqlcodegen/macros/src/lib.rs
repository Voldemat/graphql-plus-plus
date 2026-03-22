use proc_macro::TokenStream;
use quote::quote;

fn extract_scalar_from_gql_attr(
    gql_attr: &syn::Attribute,
) -> Option<syn::Type> {
    let metas = gql_attr.parse_args_with(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated).ok()?;

    for meta in metas {
        if let syn::Meta::NameValue(nv) = meta {
            if nv.path.is_ident("scalar") {
                if let syn::Expr::Path(expr_path) = nv.value {
                    return Some(syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: expr_path.path,
                    }));
                }
            }
        }
    }
    return None;
}

#[proc_macro_derive(GQLEnum, attributes(gql))]
pub fn gql_enum_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let syn::Data::Enum(enum_data) = &ast.data else {
        return quote! {
            compile_error!("Unexpected use of GQLEnum macro on a struct");
        }
        .into();
    };
    let variants = enum_data
        .variants
        .iter()
        .map(|v| &v.ident)
        .collect::<Vec<_>>();
    let from_string_arms = variants.iter().map(|ident| {
        let name_str = ident.to_string().to_uppercase();
        quote! {
            #name_str => Ok(Self::#ident),
        }
    });

    let to_str_arms = variants.iter().map(|ident| {
        let name_str = ident.to_string().to_uppercase();
        quote! {
            Self::#ident => Ok(#name_str),
        }
    });
    let Some(scalar) = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("gql"))
        .map(extract_scalar_from_gql_attr)
        .flatten()
    else {
        return quote! {
            compile_error!("GQLEnum requires #[gql(scalar=ExampleScalar)] attribute macro defined on enum");
        }.into();
    };
    quote! {
        impl libgql::executor::GQLEnum<#scalar> for #name {
            fn from_string(s: String) -> Result<Self, String> {
                match s.as_str() {
                    #(#from_string_arms)*
                    _ => Err(format!("Unexpected value {} for enum {}", s, stringify!(#name)))
                }
            }

            fn to_str(self: &Self) -> Result<&str, String> {
                match self {
                    #(#to_str_arms)*
                }
            }
        }
        impl libgql::executor::ast::ResolverValue<#scalar> for #name {
            fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, #scalar>, String> {
                libgql::executor::GQLEnum::<#scalar>::to_scalar(self).map(|s| Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(s)))
            }
        }
    }.into()
}

#[proc_macro_derive(GQLUnion, attributes(gql))]
pub fn gql_union_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let syn::Data::Enum(enum_data) = &ast.data else {
        return quote! {
            compile_error!("Unexpected use of GQLUnion macro on a struct");
        }
        .into();
    };
    let variants = enum_data
        .variants
        .iter()
        .map(|v| &v.ident)
        .collect::<Vec<_>>();
    let to_value_arms = variants.iter().map(|ident| {
        quote! {
            Self::#ident(item) => item.to_value(),
        }
    });
    let Some(scalar) = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("gql"))
        .map(extract_scalar_from_gql_attr)
        .flatten()
    else {
        return quote! {
            compile_error!("GQLUnion requires #[gql(scalar=ExampleScalar)] attribute macro defined on enum");
        }.into();
    };
    quote! {
        impl libgql::executor::ast::ResolverValue<#scalar> for #name {
            fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, #scalar>, String> {
                match self {
                    #(#to_value_arms)*
                }
            }
        }
    }.into()
}
