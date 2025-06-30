import { PathOrFileDescriptor, writeFileSync } from 'fs'
import { Actor, ActorContext } from '@/config.js'
import { renderNodes, TSActorConfig } from '../shared.js'
import { generateNodes } from './generators/main.js'

export enum ImportModule {
    GRAPHQL,
    APOLLO
}

export interface ApolloActorConfig extends TSActorConfig {
    outPath: PathOrFileDescriptor
    modulePaths: Record<ImportModule, string>
}

async function apolloActor(config: ApolloActorConfig, context: ActorContext) {
    const nodes = generateNodes(config, context)
    const code = await renderNodes(config, nodes)
    writeFileSync(config.outPath, code)
}

export function buildApolloActor(
    config: ApolloActorConfig
): Actor<ActorContext> {
    return context => apolloActor(config, context)
}
