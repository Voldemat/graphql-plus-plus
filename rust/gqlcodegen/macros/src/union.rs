use quote::quote;

pub(crate) fn gql_union_impl(ast: syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let syn::Data::Enum(enum_data) = &ast.data else {
        return quote! {
            compile_error!("Unexpected use of GQLUnion macro on struct");
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
        .map(super::shared::extract_scalar_from_gql_attr)
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
