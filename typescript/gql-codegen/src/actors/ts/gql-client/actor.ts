import { Actor, ActorContext, RunAction } from '@/config.js';
import { OperationType } from '@/schema/client/operation.js';
import { PathOrFileDescriptor } from 'fs';
import ts from 'typescript';
import { executeRunAction } from '../../utils.js';
import {
    ClientTypeNameBuilders,
    renderNodes,
    TSActorConfig,
} from '../shared.js';
import { generateNodes } from './generators/main.js';

export type OperationReturnType = 'ExecuteResult' | 'ExecuteResult.result';
export interface SDKConfig {
    defaultOperationReturnType: OperationReturnType;
    operationReturnTypeMapping: Record<string, OperationReturnType>;
    queriesKey: string;
    mutationsKey: string;
    subscriptionsKey: string;
    gqlSyncMethodFuncTypeName: string;
    gqlSubscriptionMethodFuncTypeName: string;
    operationRequestsTypeNameBuilder: (type: OperationType) => string;
    typeName: string;
    clientTypeNameBuilders: ClientTypeNameBuilders;
}
export interface Config extends TSActorConfig {
    outPath: PathOrFileDescriptor;
    importDeclarations: ts.ImportDeclaration[];
    sdk: SDKConfig;
    graphqlModulePath: string;
}

async function actor(config: Config, context: ActorContext, action: RunAction) {
    const nodes = generateNodes(config, context);
    const code = await renderNodes(config, nodes);
    executeRunAction(config.outPath, action, code);
}

export function build(config: Config): Actor<ActorContext> {
    return (context, action) => actor(config, context, action);
}
