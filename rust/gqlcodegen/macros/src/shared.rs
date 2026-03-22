pub(crate) fn extract_scalar_from_gql_attr(
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
