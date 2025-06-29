import ts from 'typescript';
import { ActorContext } from '../../../config.js';
import { GraphqlActorConfig } from '../actor.js';
import { generateServerNodes } from './server/main.js';
import { helperNodes } from './helperNodes.js';
import { generateClientNodes } from './client/main.js';

export function generateNodes(
    config: GraphqlActorConfig,
    context: ActorContext
) {
    const scalars = Object.keys(config.scalarsMapping)
    return [
        ...config.importDeclarations,
        ts.factory.createIdentifier('\n'),
        ...helperNodes,
        ts.factory.createIdentifier('\n'),
        ...generateServerNodes(config, context, scalars),
        ...generateClientNodes(config, context, scalars)
    ]
}
