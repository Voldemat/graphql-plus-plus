/// <reference types="tree-sitter-cli/dsl" />
// @ts-check
import shared from './rules.shared.js'

/**
 * @template {string} TLocalRuleName
 * @typedef {{[name in TLocalRuleName]: RuleBuilder<TLocalRuleName | keyof typeof shared>}} LocalRuleBuilders
 */
/**
 * @template {string} TRule
 * @param {LocalRuleBuilders<TRule>} rules
 * @returns RuleBuilders<TRule, never>
 */
export function helper(rules) {
    return rules
}

export default grammar({
    name: 'gql_server',
    extras: ($) => [/[\s\uFEFF\u0009\u0020\u000A\u000D]/, $.comment],
    rules: {
        ...helper({
            source_file: ($) => repeat($.node),
            node: ($) => choice($.definition_node, $.extension_node),
            extension_node: ($) =>
                seq(
                    'extend',
                    choice(
                        $.input_type_definition,
                        $.object_type_definition,
                        $.interface_type_definition,
                        $.union_type_definition,
                    ),
                ),
            definition_node: ($) =>
                choice(
                    $.scalar_definition,
                    $.enum_type_definition,
                    $.input_type_definition,
                    $.object_type_definition,
                    $.interface_type_definition,
                    $.union_type_definition,
                    $.directive_definition,
                ),
            scalar_definition: ($) => seq('scalar', $.identifier),
            input_type_definition: ($) =>
                seq(
                    'input',
                    $.identifier,
                    optional($.directive_invocations),
                    '{',
                    repeat($.input_field_definition),
                    '}',
                ),
            enum_type_definition: ($) =>
                seq(
                    'enum',
                    $.identifier,
                    optional($.directive_invocations),
                    '{',
                    repeat($.enum_value_definition),
                    '}',
                ),
            enum_value_definition: ($) => $.identifier,
            object_type_definition: ($) =>
                seq(
                    'type',
                    $.identifier,
                    optional($.implements_interfaces),
                    optional($.directive_invocations),
                    optional(seq('{', repeat($.object_field_definition), '}')),
                ),
            implements_interfaces: ($) =>
                seq(
                    'implements',
                    seq($.identifier, repeat(seq('&', $.identifier))),
                ),
            object_field_definition: ($) =>
                seq(
                    optional($.documentation),
                    $.identifier,
                    optional($.argument_definitions),
                    ':',
                    $.field_type,
                    optional($.default_value),
                    optional($.directive_invocations),
                ),
            interface_type_definition: ($) =>
                seq(
                    'interface',
                    $.identifier,
                    optional($.implements_interfaces),
                    optional($.directive_invocations),
                    '{',
                    repeat($.object_field_definition),
                    '}',
                ),
            union_type_definition: ($) =>
                seq(
                    'union',
                    $.identifier,
                    optional($.directive_invocations),
                    '=',
                    seq($.identifier, repeat(seq('|', $.identifier))),
                ),
            directive_definition: ($) =>
                seq(
                    optional($.documentation),
                    'directive',
                    '@',
                    $.identifier,
                    optional($.argument_definitions),
                    optional('repeatable'),
                    'on',
                    seq(
                        $.directive_location,
                        repeat(seq('|', $.directive_location)),
                    ),
                ),
            directive_location: () =>
                choice(
                    'SCHEMA',
                    'SCALAR',
                    'OBJECT',
                    'FIELD_DEFINITION',
                    'ARGUMENT_DEFINITION',
                    'INTERFACE',
                    'UNION',
                    'ENUM',
                    'ENUM_VALUE',
                    'INPUT_OBJECT',
                    'INPUT_FIELD_DEFINITION',
                ),
        }),
        ...shared,
    },
})
