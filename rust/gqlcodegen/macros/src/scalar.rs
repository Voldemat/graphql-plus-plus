use quote::quote;

pub(crate) fn gql_scalar_resolver_value_impl(
    input_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input_stream as syn::ItemImpl);
    let self_ty: &syn::Type = &*input.self_ty;
    let scalar_ty = if let Some((_, path, _)) = &input.trait_ {
        if let Some(last_segment) = path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) =
                &last_segment.arguments
            {
                if let Some(syn::GenericArgument::Type(ty)) = args.args.first()
                {
                    ty
                } else {
                    return syn::Error::new_spanned(
                        args,
                        "Expected scalar type",
                    )
                    .to_compile_error()
                    .into();
                }
            } else {
                return syn::Error::new_spanned(
                    last_segment,
                    "Expected generic arguments",
                )
                .to_compile_error()
                .into();
            }
        } else {
            return syn::Error::new_spanned(path, "Expected path segments")
                .to_compile_error()
                .into();
        }
    } else {
        return syn::Error::new_spanned(&input, "Expected impl of GQLScalar")
            .to_compile_error()
            .into();
    };

    quote! {
        #input

        impl libgql::executor::ast::ResolverValue<#scalar_ty> for #self_ty {
            fn to_value<'a>(
                self: &'a Self,
            ) -> Result<
                libgql::executor::ast::ResolverIntrospectionValue<'a, #scalar_ty>,
                String,
            > {
                libgql::executor::GQLScalar::<#scalar_ty>::to_scalar(self)
                    .map(|scalar| Some(
                        libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(scalar)
                    ))
            }
        }
    }.into()
}
