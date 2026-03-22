mod enum_type;
mod shared;
mod union;
mod object;
use proc_macro::TokenStream;

#[proc_macro_derive(GQLEnum, attributes(gql))]
pub fn gql_enum_derive(input_stream: TokenStream) -> TokenStream {
    enum_type::gql_enum_impl(syn::parse(input_stream).unwrap())
}

#[proc_macro_derive(GQLUnion, attributes(gql))]
pub fn gql_union_derive(input_stream: TokenStream) -> TokenStream {
    union::gql_union_impl(syn::parse(input_stream).unwrap())
}

#[proc_macro_derive(GQLObject, attributes(gql))]
pub fn gql_object_derive(input_stream: TokenStream) -> TokenStream {
    object::gql_object_impl(syn::parse(input_stream).unwrap())
}
