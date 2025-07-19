import ts from 'typescript';
import { ActorContext } from '@/config.js';
import { GraphqlActorConfig } from '../../actor.js';
import { generateFragmentTypes } from './fragments.js';
import { generateOperationsNodes } from './operations.js';

const operationTypeNode = ts.factory.createInterfaceDeclaration(
    ts.factory.createModifiersFromModifierFlags(ts.ModifierFlags.Export),
    'Operation',
    [
        ts.factory.createTypeParameterDeclaration(undefined, 'V'),
        ts.factory.createTypeParameterDeclaration(undefined, 'R')
    ],
    [],
    [
        ts.factory.createPropertySignature(
            undefined,
            'name',
            undefined,
            ts.factory.createTypeReferenceNode('string'),
        ),
        ts.factory.createPropertySignature(
            undefined,
            'type',
            undefined,
            ts.factory.createUnionTypeNode(
                ['QUERY', 'MUTATION', 'SUBSCRIPTION'].map(v =>
                    ts.factory.createLiteralTypeNode(
                        ts.factory.createStringLiteral(v)
                    ))
            )
        ),
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
            ts.factory.createTypeReferenceNode('z.ZodType', [
                ts.factory.createTypeReferenceNode('V')
            ]),
        ),
        ts.factory.createPropertySignature(
            undefined,
            'resultSchema',
            undefined,
            ts.factory.createTypeReferenceNode('z.ZodType', [
                ts.factory.createTypeReferenceNode('R')
            ]),
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
