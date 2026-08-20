/* oxlint-disable max-lines */
import { ActorContext } from '@/config.js';
import { Config } from '../../actor.js';
import ts from 'typescript';
import { generateSdkTypeNode } from './sdk-operations-type-node.js';
import { generateSdkType } from './sdk-type.js';
import {
    generateSyncHookBuilder,
    generateSyncHookType,
} from './sync-hook-builders.js';
import {
    generateLazyHookBuilder,
    generateLazyHookType,
} from './lazy-hook-builder.js';
import {
    generateSubscriptionHookBuilder,
    generateSubscriptionHookType,
} from './subscription-hook-builders.js';

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
                  ts.factory.createIdentifier('\n'),
              ]
            : []),
        ...(state.hasMutations
            ? [
                  generateSdkTypeNode(
                      config,
                      'MUTATION',
                      context.schema.client.operations,
                  ),
                  ts.factory.createIdentifier('\n'),
              ]
            : []),
        ...(state.hasSubscriptions
            ? [
                  generateSdkTypeNode(
                      config,
                      'SUBSCRIPTION',
                      context.schema.client.operations,
                  ),
                  ts.factory.createIdentifier('\n'),
              ]
            : []),
    ];
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
    return [
        ...(state.hasQueries
            ? [
                  generateSyncHookType(config),
                  ts.factory.createIdentifier('\n'),
                  generateSyncHookBuilder(config),
              ]
            : []),
        ...(state.hasQueries || state.hasMutations
            ? [
                  generateLazyHookType(config),
                  ts.factory.createIdentifier('\n'),
                  generateLazyHookBuilder(config),
              ]
            : []),
        ...(state.hasSubscriptions
            ? [
                  generateSubscriptionHookType(config),
                  ts.factory.createIdentifier('\n'),
                  generateSubscriptionHookBuilder(config),
              ]
            : []),
        ...generateSdkTypeNodes(config, context, state),
        generateSdkType(config, state),
    ];
}
