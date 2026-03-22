use quote::quote;

pub(crate) fn gql_enum_impl(ast: syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let syn::Data::Enum(enum_data) = &ast.data else {
        return quote! {
            compile_error!("Unexpected use of GQLEnum macro on struct");
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
        .map(super::shared::extract_scalar_from_gql_attr)
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
