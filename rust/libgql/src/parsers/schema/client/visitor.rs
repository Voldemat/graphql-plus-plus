use crate::parsers::schema::{
    client::{self, type_registry::TypeRegistry},
    server, shared,
};

pub type ASTVisitorHook<'a, T> = Option<Box<dyn FnMut(&T) + 'a>>;

#[derive(Default)]
pub struct ASTVisitorHooks<'a> {
    pub visit_fragment: ASTVisitorHook<'a, client::ast::Fragment>,
    pub visit_fragment_spec: ASTVisitorHook<'a, client::ast::FragmentSpec>,
    pub visit_union_fragment_spec:
        ASTVisitorHook<'a, client::ast::UnionFragmentSpec>,
    pub visit_union_selection: ASTVisitorHook<'a, client::ast::UnionSelection>,
    pub visit_typename_field: ASTVisitorHook<'a, client::ast::TypenameField>,
    pub visit_spread_selection:
        ASTVisitorHook<'a, client::ast::SpreadSelection>,
    pub visit_object_conditional_spread_selection:
        ASTVisitorHook<'a, client::ast::ObjectConditionalSpreadSelection>,
    pub visit_object_type: ASTVisitorHook<'a, server::ast::ObjectType>,
    pub visit_object_fragment_spec:
        ASTVisitorHook<'a, client::ast::ObjectFragmentSpec>,
    pub visit_interface_fragment_spec:
        ASTVisitorHook<'a, client::ast::InterfaceFragmentSpec>,
    pub visit_interface: ASTVisitorHook<'a, server::ast::Interface>,
    pub visit_object_selection:
        ASTVisitorHook<'a, client::ast::ObjectSelection>,
    pub visit_field_selection: ASTVisitorHook<'a, client::ast::FieldSelection>,
    pub visit_field_selection_argument:
        ASTVisitorHook<'a, shared::ast::FieldSelectionArgument>,
    pub visit_argument_value: ASTVisitorHook<'a, shared::ast::ArgumentValue>,
    pub visit_argument_ref_value: ASTVisitorHook<'a, String>,
    pub visit_argument_literal_value:
        ASTVisitorHook<'a, shared::ast::ArgumentLiteralValue>,
    pub visit_client_directive:
        ASTVisitorHook<'a, client::ast::ClientDirective>,
    pub visit_client_directive_location:
        ASTVisitorHook<'a, client::ast::DirectiveLocation>,
    pub visit_field_definition_input_field_spec: ASTVisitorHook<
        'a,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >,
    pub visit_field_definition_object_field_spec: ASTVisitorHook<
        'a,
        shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>,
    >,
    pub visit_non_callable_field_spec_input_type_spec: ASTVisitorHook<
        'a,
        shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec>,
    >,
    pub visit_literal_field_spec_input_type_spec: ASTVisitorHook<
        'a,
        shared::ast::LiteralFieldSpec<shared::ast::InputTypeSpec>,
    >,
    pub visit_array_field_spec_input_type_spec: ASTVisitorHook<
        'a,
        shared::ast::ArrayFieldSpec<shared::ast::InputTypeSpec>,
    >,
    pub visit_input_type_spec: ASTVisitorHook<'a, shared::ast::InputTypeSpec>,
    pub visit_non_callable_field_spec_object_type_spec: ASTVisitorHook<
        'a,
        shared::ast::NonCallableFieldSpec<server::ast::ObjectTypeSpec>,
    >,
    pub visit_literal_field_spec_object_type_spec: ASTVisitorHook<
        'a,
        shared::ast::LiteralFieldSpec<server::ast::ObjectTypeSpec>,
    >,
    pub visit_array_field_spec_object_type_spec: ASTVisitorHook<
        'a,
        shared::ast::ArrayFieldSpec<server::ast::ObjectTypeSpec>,
    >,
    pub visit_object_field_spec:
        ASTVisitorHook<'a, server::ast::ObjectFieldSpec>,
    pub visit_callable_field_spec:
        ASTVisitorHook<'a, server::ast::CallableFieldSpec>,
    pub visit_object_type_spec: ASTVisitorHook<'a, server::ast::ObjectTypeSpec>,
    pub visit_operation: ASTVisitorHook<'a, client::ast::Operation>,
    pub visit_input_type: ASTVisitorHook<'a, shared::ast::InputType>,
    pub visit_scalar: ASTVisitorHook<'a, String>,
    pub visit_enum: ASTVisitorHook<'a, shared::ast::Enum>,
    pub visit_union: ASTVisitorHook<'a, server::ast::Union>,
}

fn visit_field_selection(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    selection: &client::ast::FieldSelection,
) {
    if let Some(hook) = hooks.visit_field_selection.as_mut() {
        hook(selection);
    }
    for argument in selection.arguments.values() {
        if let Some(hook) = hooks.visit_field_selection_argument.as_mut() {
            hook(argument);
        }
        if let Some(hook) = hooks.visit_argument_value.as_mut() {
            hook(&argument.value);
        }
        match &argument.value {
            shared::ast::ArgumentValue::Ref(r) => {
                if let Some(hook) = hooks.visit_argument_ref_value.as_mut() {
                    hook(r);
                }
            }
            shared::ast::ArgumentValue::Literal(literal) => {
                if let Some(hook) = hooks.visit_argument_literal_value.as_mut()
                {
                    hook(literal);
                }
            }
        }
    }
    if let Some(s) = &selection.selection {
        visit_fragment_spec(server_registry, client_registry, hooks, s);
    }
}

fn visit_field_definition_object_field_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    field: &shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>,
) {
    if let Some(hook) = hooks.visit_field_definition_object_field_spec.as_mut()
    {
        hook(field);
    }
    visit_object_field_spec(
        server_registry,
        client_registry,
        hooks,
        &field.spec,
    )
}

fn visit_object_selection(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    selection: &client::ast::ObjectSelection,
    fields: &indexmap::IndexMap<
        String,
        shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>,
    >,
) {
    if let Some(hook) = hooks.visit_object_selection.as_mut() {
        hook(selection);
    }
    match selection {
        client::ast::ObjectSelection::TypenameField(typename_field) => {
            if let Some(hook) = hooks.visit_typename_field.as_mut() {
                hook(typename_field);
            }
        }
        client::ast::ObjectSelection::FieldSelection(field) => {
            visit_field_selection(
                server_registry,
                client_registry,
                hooks,
                field,
            );
            let f = fields.get(&field.name).unwrap();
            visit_field_definition_object_field_spec(
                server_registry,
                client_registry,
                hooks,
                f,
            );
        }
        client::ast::ObjectSelection::SpreadSelection(spread) => {
            if let Some(hook) = hooks.visit_spread_selection.as_mut() {
                hook(spread);
            }
            let fragment =
                client_registry.fragments.get(&spread.fragment).unwrap();
            if let Some(hook) = hooks.visit_fragment.as_mut() {
                hook(fragment)
            }
            visit_fragment_spec(
                server_registry,
                client_registry,
                hooks,
                &fragment.spec,
            )
        }
    }
}

fn visit_object_fragment_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    spec: &client::ast::ObjectFragmentSpec,
) {
    if let Some(hook) = hooks.visit_object_fragment_spec.as_mut() {
        hook(spec);
    }
    let object_type = server_registry.objects.get(&spec.r#type).unwrap();
    if let Some(hook) = hooks.visit_object_type.as_mut() {
        hook(&object_type);
    }
    for selection in spec.selections.iter() {
        visit_object_selection(
            server_registry,
            client_registry,
            hooks,
            selection,
            &object_type.fields,
        )
    }
}

fn visit_object_type_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    hooks: &mut ASTVisitorHooks,
    type_spec: &server::ast::ObjectTypeSpec,
) {
    if let Some(hook) = hooks.visit_object_type_spec.as_mut() {
        hook(type_spec);
    }
    match type_spec {
        server::ast::ObjectTypeSpec::Interface(interface) => {
            if let Some(hook) = hooks.visit_interface.as_mut() {
                hook(server_registry.interfaces.get(interface).unwrap());
            }
        }
        server::ast::ObjectTypeSpec::Enum(e) => {
            if let Some(hook) = hooks.visit_enum.as_mut() {
                hook(server_registry.enums.get(e).unwrap());
            }
        }
        server::ast::ObjectTypeSpec::ObjectType(o) => {
            if let Some(hook) = hooks.visit_object_type.as_mut() {
                hook(server_registry.objects.get(o).unwrap());
            }
        }
        server::ast::ObjectTypeSpec::Scalar(scalar) => {
            if let Some(hook) = hooks.visit_scalar.as_mut() {
                hook(scalar);
            }
        }
        server::ast::ObjectTypeSpec::Union(union) => {
            if let Some(hook) = hooks.visit_union.as_mut() {
                hook(server_registry.unions.get(union).unwrap());
            }
        }
    }
}

fn visit_interface_fragment_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    spec: &client::ast::InterfaceFragmentSpec,
) {
    if let Some(hook) = hooks.visit_interface_fragment_spec.as_mut() {
        hook(spec);
    }
    let interface = server_registry.interfaces.get(&spec.r#type).unwrap();
    if let Some(hook) = hooks.visit_interface.as_mut() {
        hook(interface);
    }
    for selection in spec.selections.iter() {
        visit_object_selection(
            server_registry,
            client_registry,
            hooks,
            selection,
            &interface.fields,
        )
    }
}

fn visit_non_callable_field_spec_object_type_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    field: &shared::ast::NonCallableFieldSpec<server::ast::ObjectTypeSpec>,
) {
    if let Some(hook) = hooks
        .visit_non_callable_field_spec_object_type_spec
        .as_mut()
    {
        hook(field);
    }
    match field {
        shared::ast::NonCallableFieldSpec::Literal(literal) => {
            if let Some(hook) =
                hooks.visit_literal_field_spec_object_type_spec.as_mut()
            {
                hook(literal);
            }
            visit_object_type_spec(server_registry, hooks, &literal.r#type);
        }
        shared::ast::NonCallableFieldSpec::Array(array) => {
            if let Some(hook) =
                hooks.visit_array_field_spec_object_type_spec.as_mut()
            {
                hook(array);
            }
            visit_non_callable_field_spec_object_type_spec(
                server_registry,
                client_registry,
                hooks,
                &array.r#type,
            );
        }
    }
}

fn visit_object_field_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    spec: &server::ast::ObjectFieldSpec,
) {
    if let Some(hook) = hooks.visit_object_field_spec.as_mut() {
        hook(spec);
    }
    match spec {
        server::ast::ObjectFieldSpec::Literal(literal) => {
            if let Some(hook) =
                hooks.visit_literal_field_spec_object_type_spec.as_mut()
            {
                hook(literal);
            }
            visit_object_type_spec(server_registry, hooks, &literal.r#type);
        }
        server::ast::ObjectFieldSpec::Array(array) => {
            if let Some(hook) =
                hooks.visit_array_field_spec_object_type_spec.as_mut()
            {
                hook(array);
            }
            visit_non_callable_field_spec_object_type_spec(
                server_registry,
                client_registry,
                hooks,
                &array.r#type,
            );
        }
        server::ast::ObjectFieldSpec::Callable(callable) => {
            if let Some(hook) = hooks.visit_callable_field_spec.as_mut() {
                hook(callable);
            }
            for argument in callable.arguments.values() {
                visit_field_definition_input_field_spec(
                    server_registry,
                    client_registry,
                    hooks,
                    argument,
                );
            }
            visit_non_callable_field_spec_object_type_spec(
                server_registry,
                client_registry,
                hooks,
                &callable.return_type,
            );
        }
    }
}

fn visit_union(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    union: &server::ast::Union,
) {
    if let Some(hook) = hooks.visit_union.as_mut() {
        hook(union);
    }
    for object_name in &union.items {
        let object = server_registry.objects.get(object_name).unwrap();
        if let Some(hook) = hooks.visit_object_type.as_mut() {
            hook(object);
        }
        for field in object.fields.values() {
            visit_object_field_spec(
                server_registry,
                client_registry,
                hooks,
                &field.spec,
            );
        }
    }
}

fn visit_union_selection(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    selection: &client::ast::UnionSelection,
) {
    if let Some(hook) = hooks.visit_union_selection.as_mut() {
        hook(selection);
    }
    match selection {
        client::ast::UnionSelection::TypenameField(field) => {
            if let Some(hook) = hooks.visit_typename_field.as_mut() {
                hook(field);
            }
        }
        client::ast::UnionSelection::SpreadSelection(selection) => {
            if let Some(hook) = hooks.visit_spread_selection.as_mut() {
                hook(selection);
            }
            let fragment =
                client_registry.fragments.get(&selection.fragment).unwrap();
            if let Some(hook) = hooks.visit_fragment.as_mut() {
                hook(fragment)
            }
            visit_fragment_spec(
                server_registry,
                client_registry,
                hooks,
                &fragment.spec,
            )
        }
        client::ast::UnionSelection::UnionConditionalSpreadSelection(_) => {}
        client::ast::UnionSelection::ObjectConditionalSpreadSelection(
            selection,
        ) => {
            if let Some(hook) =
                hooks.visit_object_conditional_spread_selection.as_mut()
            {
                hook(selection);
            }
            let object =
                server_registry.objects.get(&selection.r#type).unwrap();
            for selection in selection.selections.iter() {
                visit_object_selection(
                    server_registry,
                    client_registry,
                    hooks,
                    selection,
                    &object.fields,
                )
            }
        }
    }
}

fn visit_union_fragment_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    spec: &client::ast::UnionFragmentSpec,
) {
    if let Some(hook) = hooks.visit_union_fragment_spec.as_mut() {
        hook(spec);
    };
    visit_union(
        server_registry,
        client_registry,
        hooks,
        server_registry.unions.get(&spec.r#type).unwrap(),
    );
    for selection in spec.selections.iter() {
        visit_union_selection(
            server_registry,
            client_registry,
            hooks,
            selection,
        );
    }
}

fn visit_fragment_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    fragment_spec: &client::ast::FragmentSpec,
) {
    if let Some(hook) = hooks.visit_fragment_spec.as_mut() {
        hook(fragment_spec);
    }
    match fragment_spec {
        client::ast::FragmentSpec::Object(v) => {
            visit_object_fragment_spec(
                server_registry,
                client_registry,
                hooks,
                v,
            );
        }
        client::ast::FragmentSpec::Interface(v) => {
            visit_interface_fragment_spec(
                server_registry,
                client_registry,
                hooks,
                v,
            );
        }
        client::ast::FragmentSpec::Union(v) => {
            visit_union_fragment_spec(
                server_registry,
                client_registry,
                hooks,
                v,
            );
        }
    };
}

fn visit_input_type(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    t: &shared::ast::InputType,
) {
    if let Some(hook) = hooks.visit_input_type.as_mut() {
        hook(t)
    }
    for field in t.fields.values() {
        visit_field_definition_input_field_spec(
            server_registry,
            client_registry,
            hooks,
            field,
        );
    }
}

fn visit_input_type_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    spec: &shared::ast::InputTypeSpec,
) {
    if let Some(hook) = hooks.visit_input_type_spec.as_mut() {
        hook(spec)
    }
    match spec {
        shared::ast::InputTypeSpec::Enum(e) => {
            if let Some(hook) = hooks.visit_enum.as_mut() {
                hook(server_registry.enums.get(e).unwrap())
            }
        }
        shared::ast::InputTypeSpec::Scalar(s) => {
            if let Some(hook) = hooks.visit_scalar.as_mut() {
                hook(s)
            }
        }
        shared::ast::InputTypeSpec::InputType(input) => {
            visit_input_type(
                server_registry,
                client_registry,
                hooks,
                server_registry.inputs.get(input).unwrap(),
            );
        }
    }
}

fn visit_input_field_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    field_spec: &shared::ast::InputFieldSpec,
) {
    if let Some(hook) =
        hooks.visit_non_callable_field_spec_input_type_spec.as_mut()
    {
        hook(field_spec);
    }
    match field_spec {
        shared::ast::InputFieldSpec::Literal(literal) => {
            if let Some(hook) =
                hooks.visit_literal_field_spec_input_type_spec.as_mut()
            {
                hook(literal)
            }
            visit_input_type_spec(
                server_registry,
                client_registry,
                hooks,
                &literal.r#type,
            )
        }
        shared::ast::InputFieldSpec::Array(array) => {
            if let Some(hook) =
                hooks.visit_array_field_spec_input_type_spec.as_mut()
            {
                hook(array)
            }
            visit_input_field_spec(
                server_registry,
                client_registry,
                hooks,
                &array.r#type,
            )
        }
    }
}

fn visit_field_definition_input_field_spec(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
    field: &shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
) {
    if let Some(hook) = hooks.visit_field_definition_input_field_spec.as_mut() {
        hook(field);
    }
    visit_input_field_spec(server_registry, client_registry, hooks, &field.spec)
}

pub fn visit_client_schema(
    server_registry: &server::type_registry::HashMapTypeRegistry,
    client_registry: &TypeRegistry,
    hooks: &mut ASTVisitorHooks,
) {
    for directive in client_registry.directives.values() {
        if let Some(hook) = hooks.visit_client_directive.as_mut() {
            hook(directive);
        }
        if let Some(hook) = hooks.visit_client_directive_location.as_mut() {
            for location in directive.locations.iter() {
                hook(location);
            }
        }
    }
    for fragment in client_registry.fragments.values() {
        if let Some(hook) = hooks.visit_fragment.as_mut() {
            hook(fragment);
        }
        visit_fragment_spec(
            server_registry,
            client_registry,
            hooks,
            &fragment.spec,
        );
    }

    for operation in client_registry.operations.values() {
        if let Some(hook) = hooks.visit_operation.as_mut() {
            hook(operation);
        }
        for parameter in operation.parameters.values() {
            visit_field_definition_input_field_spec(
                server_registry,
                client_registry,
                hooks,
                parameter,
            );
        }
        visit_fragment_spec(
            server_registry,
            client_registry,
            hooks,
            &operation.fragment_spec,
        );
    }
}
