/// <reference types="tree-sitter-cli/dsl" />
// @ts-check
import shared from './rules.shared.js';

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
    name: 'gql_client',
    extras: ($) => [/[\s\uFEFF\u0009\u0020\u000A\u000D]/, $.comment],
    rules: {
        ...helper({
            source_file: ($) => repeat($.node),
            node: ($) =>
                choice(
                    $.operation_definition,
                    $.fragment_definition,
                    $.directive_definition,
                ),
            operation_definition: ($) =>
                seq(
                    optional($.documentation),
                    $.operation_type,
                    $.operation_name,
                    optional($.argument_definitions),
                    optional($.directive_invocations),
                    $.fragment_spec,
                ),
            operation_name: ($) => $.identifier,
            operation_type: () => choice('query', 'mutation', 'subscription'),
            fragment_definition: ($) =>
                seq(
                    optional($.documentation),
                    'fragment',
                    $.fragment_name,
                    optional($.directive_invocations),
                    'on',
                    $.fragment_type,
                    $.fragment_spec,
                ),
            fragment_name: ($) => $.identifier,
            fragment_type: ($) => $.identifier,
            fragment_spec: ($) => seq('{', repeat($.fragment_selection), '}'),
            fragment_selection: ($) =>
                choice(
                    $.field_selection,
                    $.spread_selection,
                    $.conditional_spread_selection,
                ),
            field_selection: ($) =>
                seq(
                    $.identifier,
                    optional(seq(':', $.identifier)),
                    optional($.arguments),
                    optional($.fragment_spec),
                ),
            spread_selection: ($) => seq('...', $.identifier),
            conditional_spread_selection: ($) =>
                seq('...', 'on', $.identifier, $.fragment_spec),
            directive_definition: ($) =>
                seq(
                    optional($.documentation),
                    'directive',
                    '@',
                    $.identifier,
                    optional($.arguments),
                    optional('repeatable'),
                    'on',
                    seq(
                        $.directive_location,
                        repeat(seq('|', $.directive_location)),
                    ),
                ),
            directive_location: () =>
                choice(
                    'QUERY',
                    'MUTATION',
                    'SUBSCRIPTION',
                    'FIELD',
                    'FRAGMENT_DEFINITION',
                    'FRAGMENT_SPREAD',
                    'INLINE_FRAGMENT',
                    'VARIABLE_DEFINITION',
                ),
        }),
        ...shared,
    },
})
