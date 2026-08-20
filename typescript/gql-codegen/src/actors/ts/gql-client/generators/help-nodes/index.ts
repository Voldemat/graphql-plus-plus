/* oxlint-disable max-lines */
import { ActorContext } from '@/config.js';
import { Config, OperationReturnType } from '../../actor.js';
import ts from 'typescript';
import { generateSdkTypeNode } from './sdk-operations-type-node.js';
import { generateMethodFuncAlias } from './method-func-alias.js';
import { generateSdkType } from './sdk-type.js';
import { generateBuildSyncCallbackFunction } from './build-sync-callback-function.js';
import { generateBuildSubscriptionCallbackFunction } from './build-subscription-callback-function.js';

function generateSdkTypeNodes(
    config: Config,
    context: ActorContext,
    state: {
        hasQueries: boolean;
        hasMutations: boolean;
        hasSubscriptions: boolean;
    },
): ts.Node[] {
    return [
        ...(state.hasQueries
            ? [
                  generateSdkTypeNode(
                      config,
                      'QUERY',
                      context.schema.client.operations,
                  ),
              ]
            : []),
        ...(state.hasMutations
            ? [
                  generateSdkTypeNode(
                      config,
                      'MUTATION',
                      context.schema.client.operations,
                  ),
              ]
            : []),
        ...(state.hasSubscriptions
            ? [
                  generateSdkTypeNode(
                      config,
                      'SUBSCRIPTION',
                      context.schema.client.operations,
                  ),
              ]
            : []),
    ];
}

export function hasReturnType(
    config: Config,
    returnType: OperationReturnType,
): boolean {
    return (
        config.sdk.defaultOperationReturnType == returnType ||
        Object.values(config.sdk.operationReturnTypeMapping).includes(
            returnType,
        )
    );
}

export function generateHelpNodes(
    config: Config,
    context: ActorContext,
    state: {
        hasQueries: boolean;
        hasMutations: boolean;
        hasSubscriptions: boolean;
    },
): ts.Node[] {
    const hasExecuteResultType = hasReturnType(config, 'ExecuteResult');
    const hasResultType = hasReturnType(config, 'ExecuteResult.result');
    return [
        generateMethodFuncAlias(config),
        ...generateSdkTypeNodes(config, context, state),
        generateSdkType(config, state),
        ...(state.hasQueries || state.hasMutations
            ? [
                  ...(hasExecuteResultType
                      ? [
                            generateBuildSyncCallbackFunction(
                                config,
                                'ExecuteResult',
                            ),
                        ]
                      : []),
                  ...(hasResultType
                      ? [
                            generateBuildSyncCallbackFunction(
                                config,
                                'ExecuteResult.result',
                            ),
                        ]
                      : []),
              ]
            : []),
        ...(state.hasSubscriptions
            ? [
                  ...(hasExecuteResultType
                      ? [
                            generateBuildSubscriptionCallbackFunction(
                                config,
                                'ExecuteResult',
                            ),
                        ]
                      : []),
                  ...(hasResultType
                      ? [
                            generateBuildSubscriptionCallbackFunction(
                                config,
                                'ExecuteResult.result',
                            ),
                        ]
                      : []),
              ]
            : []),
    ];
}
