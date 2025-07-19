import { PathOrFileDescriptor, writeFileSync } from 'fs'
import { Actor, ActorContext } from '@/config.js'
import { renderNodes, TSActorConfig } from '../shared.js'
import { generateNodes } from './generators/main.js'
import ts from 'typescript'

export interface GQLClientReactActorConfig extends TSActorConfig {
    outPath: PathOrFileDescriptor
    importDeclarations: ts.ImportDeclaration[]
    graphqlModulePath: string
    sdk: {
        queriesKey: string
        mutationsKey: string
        subscriptionsKey: string
    }
}
async function gqlClientReactActor(
    config: GQLClientReactActorConfig,
    context: ActorContext
) {
    const nodes = generateNodes(config, context)
    const code = await renderNodes(config, nodes)
    writeFileSync(config.outPath, code)
}

export function buildGQLClientReactActor(
    config: GQLClientReactActorConfig
): Actor<ActorContext> {
    return context => gqlClientReactActor(config, context)
}
