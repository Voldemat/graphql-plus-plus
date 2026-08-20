import { PathOrFileDescriptor, writeFileSync } from 'fs';
import ts from 'typescript';
import { Actor, ActorContext } from '@/config.js';
import {
    ClientTypeNameBuilders,
    renderNodes,
    TSActorConfig,
} from '../shared.js';
import { generateNodes } from './generators/main.js';
import { Operation } from '@/schema/client/operation.js';

export type OperationReturnType = 'ExecuteResult' | 'ExecuteResult.result';
export interface SDKConfig {
    defaultOperationReturnType: OperationReturnType;
    operationReturnTypeMapping: Record<string, OperationReturnType>;
    queriesKey: string;
    mutationsKey: string;
    subscriptionsKey: string;
    gqlMethodFuncTypeName: string;
    operationRequestsTypeNameBuilder: (type: Operation['type']) => string;
    typeName: string;
    clientTypeNameBuilders: ClientTypeNameBuilders;
}
export interface Config extends TSActorConfig {
    outPath: PathOrFileDescriptor;
    importDeclarations: ts.ImportDeclaration[];
    sdk: SDKConfig;
    graphqlModulePath: string;
}

async function actor(config: Config, context: ActorContext) {
    const nodes = generateNodes(config, context);
    const code = await renderNodes(config, nodes);
    writeFileSync(config.outPath, code);
}

export function build(config: Config): Actor<ActorContext> {
    return (context) => actor(config, context);
}
