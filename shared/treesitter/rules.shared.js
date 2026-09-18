/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

/**
 * @template {string} TRule
 * @param {RuleBuilders<TRule, never>} rules
 * @returns RuleBuilders<TRule, never>
 */
export function helper(rules) {
    return rules
}

export default helper({
    identifier: () => /[_A-Za-z][_0-9A-Za-z]*/,
    argument_definitions: ($) =>
        seq(
            '(',
            optional(
                seq(
                    $.input_field_definition,
                    repeat(seq(',', $.input_field_definition)),
                    optional(','),
                ),
            ),
            ')',
        ),
    input_field_definition: ($) =>
        seq(
            optional($.documentation),
            $.identifier,
            ':',
            $.field_type,
            optional($.default_value),
            optional($.directive_invocations),
        ),
    documentation: ($) => $.identifier,
    directive_invocations: ($) => repeat1($.directive_invocation),
    directive_invocation: ($) => seq('@', $.identifier, optional($.arguments)),
    arguments: ($) => seq('(', repeat($.argument), ')'),
    argument: ($) => seq($.identifier, ':', $.argument_value),
    argument_value: ($) => choice($.argument_ref_value, $.literal_value),
    argument_ref_value: ($) => seq('$', $.identifier),
    field_type: ($) => choice($.named_type, $.list_type),
    named_type: ($) => seq($.identifier, optional('!')),
    list_type: ($) => seq('[', $.field_type, ']', optional('!')),
    default_value: ($) => seq('=', $.literal_value),
    literal_value: ($) =>
        choice(
            $.string_literal,
            $.int_literal,
            $.float_literal,
            $.boolean_literal,
            $.null_literal,
            $.enum_literal,
        ),
    enum_literal: ($) => $.identifier,
    string_literal: () =>
        choice(
            seq('"""', /([^"]|\n|""?[^"])*/, '"""'),
            seq('"', /[^"\\\n]*/, '"'),
        ),
    int_literal: () => /-?(0|[1-9][0-9]*)/,
    float_literal: () =>
        token(
            seq(
                /-?(0|[1-9][0-9]*)/,
                choice(
                    /\.[0-9]+/,
                    /(e|E)(\+|-)?[0-9]+/,
                    seq(/\.[0-9]+/, /(e|E)(\+|-)?[0-9]+/),
                ),
            ),
        ),
    boolean_literal: () => choice('true', 'false'),
    null_literal: () => 'null',
})
