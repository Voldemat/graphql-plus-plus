use std::{cell::RefCell, rc::Rc};

use crate::parsers::schema::{client, server, shared};

pub type ASTVisitorHook<'a, T> = Option<Box<dyn FnMut(&T) + 'a>>;

#[derive(Default)]
pub struct ASTVisitorHooks<'a> {
    pub visit_fragment: ASTVisitorHook<'a, Rc<RefCell<client::ast::Fragment>>>,
    pub visit_fragment_spec: ASTVisitorHook<'a, client::ast::FragmentSpec>,
    pub visit_union_fragment_spec:
        ASTVisitorHook<'a, client::ast::UnionFragmentSpec>,
    pub visit_union_selection: ASTVisitorHook<'a, client::ast::UnionSelection>,
    pub visit_typename_field: ASTVisitorHook<'a, client::ast::TypenameField>,
    pub visit_spread_selection:
        ASTVisitorHook<'a, client::ast::SpreadSelection>,
    pub visit_object_conditional_spread_selection:
        ASTVisitorHook<'a, client::ast::ObjectConditionalSpreadSelection>,
    pub visit_object_type:
        ASTVisitorHook<'a, Rc<RefCell<server::ast::ObjectType>>>,
    pub visit_object_fragment_spec_object_type: ASTVisitorHook<
        'a,
        client::ast::ObjectFragmentSpec<server::ast::ObjectType>,
    >,
    pub visit_object_fragment_spec_interface: ASTVisitorHook<
        'a,
        client::ast::ObjectFragmentSpec<server::ast::Interface>,
    >,
    pub visit_interface:
        ASTVisitorHook<'a, Rc<RefCell<server::ast::Interface>>>,
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
        ASTVisitorHook<'a, Rc<client::ast::ClientDirective>>,
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
    pub visit_operation:
        ASTVisitorHook<'a, Rc<RefCell<client::ast::Operation>>>,
    pub visit_input_type:
        ASTVisitorHook<'a, Rc<RefCell<shared::ast::InputType>>>,
    pub visit_scalar: ASTVisitorHook<'a, String>,
    pub visit_enum: ASTVisitorHook<'a, Rc<RefCell<shared::ast::Enum>>>,
    pub visit_union: ASTVisitorHook<'a, Rc<RefCell<server::ast::Union>>>,
}

fn visit_field_selection(
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
        visit_fragment_spec(hooks, s);
    }
}

fn visit_field_definition_object_field_spec(
    hooks: &mut ASTVisitorHooks,
    field: &shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>,
) {
    if let Some(hook) = hooks.visit_field_definition_object_field_spec.as_mut()
    {
        hook(field);
    }
    visit_object_field_spec(hooks, &field.spec)
}

fn visit_object_selection(
    hooks: &mut ASTVisitorHooks,
    selection: &client::ast::ObjectSelection,
    fields: &indexmap::IndexMap<
        String,
        Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
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
            visit_field_selection(hooks, field);
            let f = fields.get(&field.name).unwrap();
            visit_field_definition_object_field_spec(hooks, f);
        }
        client::ast::ObjectSelection::SpreadSelection(spread) => {
            if let Some(hook) = hooks.visit_spread_selection.as_mut() {
                hook(spread);
            }
            if let Some(hook) = hooks.visit_fragment.as_mut() {
                hook(&spread.fragment)
            }
            visit_fragment_spec(hooks, &spread.fragment.borrow().spec)
        }
    }
}

fn visit_object_fragment_spec_object_type(
    hooks: &mut ASTVisitorHooks,
    spec: &client::ast::ObjectFragmentSpec<server::ast::ObjectType>,
) {
    if let Some(hook) = hooks.visit_object_fragment_spec_object_type.as_mut() {
        hook(spec);
    }
    if let Some(hook) = hooks.visit_object_type.as_mut() {
        hook(&spec.r#type);
    }
    for selection in spec.selections.iter() {
        visit_object_selection(hooks, selection, &spec.r#type.borrow().fields)
    }
}

fn visit_object_type_spec(
    hooks: &mut ASTVisitorHooks,
    type_spec: &server::ast::ObjectTypeSpec,
) {
    if let Some(hook) = hooks.visit_object_type_spec.as_mut() {
        hook(type_spec);
    }
    match type_spec {
        server::ast::ObjectTypeSpec::Interface(interface) => {
            if let Some(hook) = hooks.visit_interface.as_mut() {
                hook(interface);
            }
        }
        server::ast::ObjectTypeSpec::Enum(e) => {
            if let Some(hook) = hooks.visit_enum.as_mut() {
                hook(e);
            }
        }
        server::ast::ObjectTypeSpec::ObjectType(o) => {
            if let Some(hook) = hooks.visit_object_type.as_mut() {
                hook(o);
            }
        }
        server::ast::ObjectTypeSpec::Scalar { name } => {
            if let Some(hook) = hooks.visit_scalar.as_mut() {
                hook(name);
            }
        }
        server::ast::ObjectTypeSpec::Union(union) => {
            if let Some(hook) = hooks.visit_union.as_mut() {
                hook(union);
            }
        }
    }
}

fn visit_object_fragment_spec_interface_type(
    hooks: &mut ASTVisitorHooks,
    spec: &client::ast::ObjectFragmentSpec<server::ast::Interface>,
) {
    if let Some(hook) = hooks.visit_object_fragment_spec_interface.as_mut() {
        hook(spec);
    }
    for selection in spec.selections.iter() {
        visit_object_selection(hooks, selection, &spec.r#type.borrow().fields)
    }
}

fn visit_non_callable_field_spec_object_type_spec(
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
            visit_object_type_spec(hooks, &literal.r#type);
        }
        shared::ast::NonCallableFieldSpec::Array(array) => {
            if let Some(hook) =
                hooks.visit_array_field_spec_object_type_spec.as_mut()
            {
                hook(array);
            }
            visit_object_type_spec(hooks, &array.r#type);
        }
    }
}

fn visit_object_field_spec(
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
            visit_object_type_spec(hooks, &literal.r#type);
        }
        server::ast::ObjectFieldSpec::Array(array) => {
            if let Some(hook) =
                hooks.visit_array_field_spec_object_type_spec.as_mut()
            {
                hook(array);
            }
            visit_object_type_spec(hooks, &array.r#type);
        }
        server::ast::ObjectFieldSpec::Callable(callable) => {
            if let Some(hook) = hooks.visit_callable_field_spec.as_mut() {
                hook(callable);
            }
            for argument in callable.arguments.values() {
                visit_field_definition_input_field_spec(hooks, argument);
            }
            visit_non_callable_field_spec_object_type_spec(
                hooks,
                &callable.return_type,
            );
        }
    }
}

fn visit_union(
    hooks: &mut ASTVisitorHooks,
    union: &Rc<RefCell<server::ast::Union>>,
) {
    if let Some(hook) = hooks.visit_union.as_mut() {
        hook(union);
    }
    for object in union.borrow().items.values() {
        if let Some(hook) = hooks.visit_object_type.as_mut() {
            hook(object);
        }
        for field in object.borrow().fields.values() {
            visit_object_field_spec(hooks, &field.spec);
        }
    }
}

fn visit_union_selection(
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
            if let Some(hook) = hooks.visit_fragment.as_mut() {
                hook(&selection.fragment)
            }
            visit_fragment_spec(hooks, &selection.fragment.borrow().spec)
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
            visit_object_fragment_spec_object_type(hooks, &selection.selection)
        }
    }
}

fn visit_union_fragment_spec(
    hooks: &mut ASTVisitorHooks,
    spec: &client::ast::UnionFragmentSpec,
) {
    if let Some(hook) = hooks.visit_union_fragment_spec.as_mut() {
        hook(spec);
    };
    visit_union(hooks, &spec.r#type);
    for selection in spec.selections.iter() {
        visit_union_selection(hooks, selection);
    }
}

fn visit_fragment_spec(
    hooks: &mut ASTVisitorHooks,
    fragment_spec: &client::ast::FragmentSpec,
) {
    if let Some(hook) = hooks.visit_fragment_spec.as_mut() {
        hook(fragment_spec);
    }
    match fragment_spec {
        client::ast::FragmentSpec::Object(v) => {
            visit_object_fragment_spec_object_type(hooks, v);
        }
        client::ast::FragmentSpec::Interface(v) => {
            visit_object_fragment_spec_interface_type(hooks, v);
        }
        client::ast::FragmentSpec::Union(v) => {
            visit_union_fragment_spec(hooks, v);
        }
    };
}

fn visit_input_type(
    hooks: &mut ASTVisitorHooks,
    t: &Rc<RefCell<shared::ast::InputType>>,
) {
    if let Some(hook) = hooks.visit_input_type.as_mut() {
        hook(t)
    }
    for field in t.borrow().fields.values() {
        visit_field_definition_input_field_spec(hooks, field);
    }
}

fn visit_input_type_spec(
    hooks: &mut ASTVisitorHooks,
    spec: &shared::ast::InputTypeSpec,
) {
    if let Some(hook) = hooks.visit_input_type_spec.as_mut() {
        hook(spec)
    }
    match spec {
        shared::ast::InputTypeSpec::Enum(e) => {
            if let Some(hook) = hooks.visit_enum.as_mut() {
                hook(e)
            }
        }
        shared::ast::InputTypeSpec::Scalar(s) => {
            if let Some(hook) = hooks.visit_scalar.as_mut() {
                hook(s)
            }
        }
        shared::ast::InputTypeSpec::InputType(input) => {
            visit_input_type(hooks, input);
        }
    }
}

fn visit_input_field_spec(
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
            visit_input_type_spec(hooks, &literal.r#type)
        }
        shared::ast::InputFieldSpec::Array(array) => {
            if let Some(hook) =
                hooks.visit_array_field_spec_input_type_spec.as_mut()
            {
                hook(array)
            }
            visit_input_type_spec(hooks, &array.r#type)
        }
    }
}

fn visit_field_definition_input_field_spec(
    hooks: &mut ASTVisitorHooks,
    field: &shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
) {
    if let Some(hook) = hooks.visit_field_definition_input_field_spec.as_mut() {
        hook(field);
    }
    visit_input_field_spec(hooks, &field.spec)
}

pub fn visit_client_schema(
    hooks: &mut ASTVisitorHooks,
    schema: &client::schema::ClientSchema,
) {
    for directive in schema.directives.values() {
        if let Some(hook) = hooks.visit_client_directive.as_mut() {
            hook(directive);
        }
        if let Some(hook) = hooks.visit_client_directive_location.as_mut() {
            for location in directive.locations.iter() {
                hook(location);
            }
        }
    }
    for fragment in schema.fragments.values() {
        if let Some(hook) = hooks.visit_fragment.as_mut() {
            hook(fragment);
        }
        visit_fragment_spec(hooks, &fragment.borrow().spec);
    }

    for operation in schema.operations.values() {
        if let Some(hook) = hooks.visit_operation.as_mut() {
            hook(operation);
        }
        for parameter in operation.borrow().parameters.values() {
            visit_field_definition_input_field_spec(hooks, parameter);
        }
        visit_fragment_spec(hooks, &operation.borrow().fragment_spec);
    }
}
