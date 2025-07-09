import ts from 'typescript';
import { ActorContext } from '@/config.js';
import { GraphqlActorConfig } from '../../actor.js';
import { generateFragmentTypes } from './fragments.js';
import { generateOperationsNodes } from './operations.js';

const operationTypeNode = ts.factory.createInterfaceDeclaration(
    ts.factory.createModifiersFromModifierFlags(ts.ModifierFlags.Export),
    'Operation',
    [],
    [],
    [
        ts.factory.createPropertySignature(
            undefined,
            'document',
            undefined,
            ts.factory.createTypeReferenceNode('string'),
        ),
        ts.factory.createPropertySignature(
            undefined,
            'variablesSchema',
            undefined,
            ts.factory.createTypeReferenceNode('z.ZodType'),
        ),
        ts.factory.createPropertySignature(
            undefined,
            'resultSchema',
            undefined,
            ts.factory.createTypeReferenceNode('z.ZodType'),
        )
    ]
)
export function generateClientNodes(
    config: GraphqlActorConfig,
    context: ActorContext,
): ts.Node[] {
    return [
        operationTypeNode,
        ...generateFragmentTypes(
            config.scalarsMapping,
            context.schema
        ),
        ...generateOperationsNodes(
            config.scalarsMapping,
            context.schema,
        )
    ]
}
