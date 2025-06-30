import ts from 'typescript';
import { RootSchema } from '../../../../schema/root.js';
import { operationSchema } from '../../../../schema/client/operation.js';
import { z } from 'zod/v4';
import { generateInputFieldsPropertySignatures } from '../server/inputs.js';
import { generateFragmentObjectSpecPropertySignatures } from './fragments.js';
import assert from 'assert';


export function generateOperationInputDataNodes(
    scalars: string[],
    operation: z.infer<typeof operationSchema>
): ts.Node[] {
    const pSignatures = generateInputFieldsPropertySignatures(
        scalars,
        Object.fromEntries(Object.keys(operation.parameters).map(name =>
            [name.slice(1), operation.parameters[name]]))
    )
    return [
        ts.factory.createTypeAliasDeclaration(
            ts.factory.createModifiersFromModifierFlags(
                ts.ModifierFlags.Export
            ),
            operation.name + 'Variables',
            undefined,
            ts.factory.createTypeReferenceNode(
                'Exact',
                [ts.factory.createTypeLiteralNode(pSignatures)]
            ),
        ),
        ts.factory.createInterfaceDeclaration(
            ts.factory.createModifiersFromModifierFlags(
                ts.ModifierFlags.Export
            ),
            operation.name + 'Args',
            undefined,
            undefined,
            pSignatures
        )
    ]
}

function opTypeToName(type: z.infer<typeof operationSchema>['type']): string {
    switch (type) {
    case 'QUERY': return 'Query'
    case 'MUTATION': return 'Mutation'
    case 'SUBSCRIPTION': return 'Subscription'
    }
}

function generateOperationReturnTypeNode(
    scalars: string[],
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>
): ts.InterfaceDeclaration {
    assert(operation.fragmentSpec._type === 'object')
    return ts.factory.createInterfaceDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        operation.name + opTypeToName(operation.type),
        undefined,
        undefined,
        generateFragmentObjectSpecPropertySignatures(
            scalars,
            schema,
            operation.fragmentSpec,
            { ensurePresent: true, optional: true, ignore: false }
        )
    )
}

export function generateOperationNodes(
    scalars: string[],
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>
): ts.Node[] {
    return [
        ...generateOperationInputDataNodes(scalars, operation),
        generateOperationReturnTypeNode(scalars, schema, operation)
    ]
}

export function generateOperationsNodes(
    scalars: string[],
    schema: RootSchema,
): ts.Node[] {
    return Object.values(schema.client.operations).map(operation => {
        return generateOperationNodes(scalars, schema, operation)
    }).flat()
}
