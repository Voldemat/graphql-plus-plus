use quote::quote;

fn extract_field_name(
    attr: &syn::Attribute,
) -> Result<Option<String>, syn::Error> {
    let metas = attr.parse_args_with(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)?;

    for meta in metas {
        if let syn::Meta::NameValue(nv) = meta {
            if nv.path.is_ident("name") {
                if let syn::Expr::Lit(expr_lit) = &nv.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        return Ok(Some(lit_str.value()));
                    }
                    return Err(syn::Error::new_spanned(
                        nv,
                        "Expected #[gql(name = \"...\")]",
                    ));
                }
            }
        }
    }
    return Ok(None);
}

fn extract_field_name_from_attrs(
    attrs: &[syn::Attribute],
) -> Result<Option<String>, syn::Error> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("gql"))
        .map(extract_field_name)
        .transpose()
        .map(|v| v.flatten())
}

pub(crate) fn gql_object_impl(
    ast: syn::DeriveInput,
) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let syn::Data::Struct(struct_data) = &ast.data else {
        return quote! {
            compile_error!("Unexpected use of GQLObject macro on enum");
        }
        .into();
    };
    let Some(scalar) = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("gql"))
        .map(super::shared::extract_scalar_from_gql_attr)
        .flatten()
    else {
        return quote! {
            compile_error!("GQLObject requires #[gql(scalar=ExampleScalar)] attribute macro defined on struct");
        }.into();
    };
    let field_entries = match struct_data.fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let name_str = extract_field_name_from_attrs(&field.attrs)?
            .unwrap_or_else(||ident.to_string());

        Ok(quote! {
            (#name_str, &self.#ident as &libgql::executor::ast::ResolverRoot<#scalar>)
        })
    }).collect::<Result<Vec<_>, syn::Error>>() {
        Ok(e) => e,
        Err(e) => return e.to_compile_error().into()
    };
    quote! {
        impl libgql::executor::ast::ResolverValue<#scalar> for #name {
            fn to_value<'a>(self: &'a Self) -> Result<libgql::executor::ast::ResolverIntrospectionValue<'a, #scalar>, String> {
                Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(
                    self,
                    stringify!(#name),
                    std::collections::HashMap::from_iter([
                        #(#field_entries),*
                    ])
                )))
            }
        }
    }.into()
}
