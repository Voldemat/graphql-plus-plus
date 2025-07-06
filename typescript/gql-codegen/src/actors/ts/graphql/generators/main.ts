import ts from 'typescript';
import { ActorContext } from '@/config.js';
import { GraphqlActorConfig } from '../actor.js';
import { generateServerNodes } from './server/main.js';
import { generateClientNodes } from './client/main.js';

export function generateNodes(
    config: GraphqlActorConfig,
    context: ActorContext
) {
    const scalars = Object.keys(config.scalarsMapping)
    return [
        ts.factory.createImportDeclaration(
            [],
            ts.factory.createImportClause(
                false,
                undefined,
                ts.factory.createNamedImports([
                    ts.factory.createImportSpecifier(
                        false,
                        undefined,
                        ts.factory.createIdentifier('z')
                    )
                ])
            ),
            ts.factory.createStringLiteral('zod/v4')
        ),
        ...config.importDeclarations,
        ts.factory.createIdentifier('\n'),
        ...generateServerNodes(config, context, config.scalarsMapping),
        ...generateClientNodes(config, context, scalars)
    ]
}
