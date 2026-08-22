import { Actor, ActorContext } from '@/config.js';
import { OperationType } from '@/schema/client/operation.js';
import { PathOrFileDescriptor, writeFileSync } from 'fs';
import ts from 'typescript';
import {
    ClientTypeNameBuilders,
    renderNodes,
    TSActorConfig,
} from '../shared.js';
import { generateNodes } from './generators/main.js';

export interface HookNameBuilders {
    query: {
        immediate: (operationName: string) => string;
        lazy: (operationName: string) => string;
    };
    mutation: {
        lazy: (operationName: string) => string;
    };
    subscription: {
        immediate: (operationName: string) => string;
    };
}

export interface SDKConfig {
    typeName: string;
    queriesKey: string;
    mutationsKey: string;
    subscriptionsKey: string;
    clientTypeNameBuilders: ClientTypeNameBuilders;
    hookNameBuilders: HookNameBuilders;
    operationHooksTypeNameBuilder: (type: OperationType) => string;
    lazyHookBuilderName: string;
    lazyHookTypeName: string;
    syncHookBuilderName: string;
    syncHookTypeName: string;
    subscriptionHookBuilderName: string;
    subscriptionHookTypeName: string;
    buildVariablesType: (
        variablesTypeNode: ts.TypeReferenceNode,
    ) => ts.TypeNode;
    buildRequestContextType: (
        requestContextTypeNode: ts.TypeReferenceNode,
    ) => ts.TypeNode;
}

export interface Config extends TSActorConfig {
    outPath: PathOrFileDescriptor;
    importDeclarations: ts.ImportDeclaration[];
    frameworkImportName: string;
    graphqlModulePath: string;
    sdk: SDKConfig;
}
async function actor(config: Config, context: ActorContext) {
    const nodes = generateNodes(config, context);
    const code = await renderNodes(config, nodes);
    writeFileSync(config.outPath, code);
}

export function build(config: Config): Actor<ActorContext> {
    return (context) => actor(config, context);
}
