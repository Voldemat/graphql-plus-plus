use crate::parsers::file::shared;

enum DirectiveLocation {
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
}

type DirectiveLocationNode =
    shared::ast::DirectiveLocationNode<DirectiveLocation>;
type DirectiveDefinitionNode = shared::ast::DirectiveNode<DirectiveLocation>;

struct FieldDefinitionNode {
    location: shared::ast::NodeLocation,
    name: shared::ast::NameNode,
    r#type: shared::ast::TypeNode,
    arguments: Vec<shared::ast::InputValueDefinitionNode>,
    directives: Vec<shared::ast::DirectiveInvocationNode>
}

struct InterfaceDefinitionNode {
    location: shared::ast::NodeLocation,
    name: shared::ast::NameNode,
    fields: Vec<FieldDefinitionNode>,
    directives: Vec<shared::ast::DirectiveInvocationNode>
}

struct ObjectDefinitionNode {
    location: shared::ast::NodeLocation,
    name: shared::ast::NameNode,
    interfaces: Vec<shared::ast::NameNode>,
    fields: Vec<FieldDefinitionNode>,
    directives: Vec<shared::ast::DirectiveInvocationNode>
}

struct InputObjectDefinitionNode {
    location: shared::ast::NodeLocation,
    name: shared::ast::NameNode,
    fields: Vec<FieldDefinitionNode>,
    directives: Vec<shared::ast::DirectiveInvocationNode>
}

struct EnumValueDefinitionNode {
    location: shared::ast::NodeLocation,
    value: shared::ast::NameNode,
    directives: Vec<shared::ast::DirectiveInvocationNode>
}

struct EnumDefinitionNode {
    location: shared::ast::NodeLocation,
    name: shared::ast::NameNode,
    values: Vec<EnumValueDefinitionNode>,
    directives: Vec<shared::ast::DirectiveInvocationNode>,
}

struct UnionDefinitionNode {
    location: shared::ast::NodeLocation,
    name: shared::ast::NameNode,
    values: Vec<shared::ast::NameNode>,
    directives: Vec<shared::ast::DirectiveInvocationNode>,
}

struct ScalarDefinitionNode {
    location: shared::ast::NodeLocation,
    name: shared::ast::NameNode,
}

enum TypeDefinitionNode {
    Scalar(ScalarDefinitionNode),
    Union(UnionDefinitionNode),
    Enum(EnumDefinitionNode),
    Input(InputObjectDefinitionNode),
    Object(ObjectDefinitionNode),
    Interface(InterfaceDefinitionNode),
    Directive(DirectiveDefinitionNode)
}

struct ExtendTypeNode {
    location: shared::ast::NodeLocation,
    type_node: ObjectDefinitionNode
}

enum ASTNode {
    TypeDefinitionNode(TypeDefinitionNode),
    ExtendTypeNode(ExtendTypeNode)
}
