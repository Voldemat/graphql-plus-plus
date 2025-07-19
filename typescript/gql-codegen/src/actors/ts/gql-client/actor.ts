import { PathOrFileDescriptor, writeFileSync } from 'fs'
import ts from 'typescript'
import { Actor, ActorContext } from '@/config.js'
import { renderNodes, TSActorConfig } from '../shared.js'
import { generateNodes } from './generators/main.js'

export type OperationReturnType = 'ExecuteResult' | 'ExecuteResult.result'
export interface SDKConfig {
    defaultOperationReturnType: OperationReturnType
    operationReturnTypeMapping: Record<string, OperationReturnType>
    queriesKey: string
    mutationsKey: string
    subscriptionsKey: string
}
export interface GQLClientActorConfig extends TSActorConfig {
    outPath: PathOrFileDescriptor
    importDeclarations: ts.ImportDeclaration[]
    sdk: SDKConfig
    graphqlModulePath: string
}

async function gqlActor(
    config: GQLClientActorConfig,
    context: ActorContext
) {
    const nodes = generateNodes(config, context)
    const code = await renderNodes(config, nodes)
    writeFileSync(config.outPath, code)
}

export function buildGQLClientActor(
    config: GQLClientActorConfig
): Actor<ActorContext> {
    return context => gqlActor(config, context)
}
