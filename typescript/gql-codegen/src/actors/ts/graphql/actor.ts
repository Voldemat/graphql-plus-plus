import { ScalarsMapping } from './generators/server/scalars/index.js'
import { PathOrFileDescriptor, writeFileSync } from 'fs'
import ts from 'typescript'
import { Actor, ActorContext } from '@/config.js'
import { renderNodes, TSActorConfig } from '../shared.js'
import { generateNodes } from './generators/main.js'

export interface GraphqlActorConfig extends TSActorConfig {
    outPath: PathOrFileDescriptor
    scalarsMapping: ScalarsMapping
    importDeclarations: ts.ImportDeclaration[]
    onlyRequiredForOperations: boolean
}

async function graphqlActor(config: GraphqlActorConfig, context: ActorContext) {
    const nodes = generateNodes(config, context)
    const code = await renderNodes(config, nodes)
    writeFileSync(config.outPath, code)
}

export function buildGraphqlActor(
    config: GraphqlActorConfig
): Actor<ActorContext> {
    return context => graphqlActor(config, context)
}
