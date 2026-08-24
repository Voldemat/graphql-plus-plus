import { ClientTypeNameBuilders } from '@/actors/ts/shared.js';
import { operationSchema } from '@/schema/client/operation.js';
import { RootSchema } from '@/schema/root.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { ScalarsMapping } from '../../server/scalars/mapping.js';
import {
    generateSchemaName,
    generateZodInferInterfaceType,
} from '../../server/shared.js';
import { generateOperationZodInputSchema } from './input-schema.js';
import { generateOperationNode } from './node.js';
import { generateOperationZodOutputSchema } from './output-schema.js';

export function opTypeToName(
    type: z.infer<typeof operationSchema>['type'],
): string {
    switch (type) {
        case 'QUERY':
            return 'Query';
        case 'MUTATION':
            return 'Mutation';
        case 'SUBSCRIPTION':
            return 'Subscription';
    }
}

function generateOperationNodes(
    clientTypeNameBuilders: ClientTypeNameBuilders,
    scalarsMapping: ScalarsMapping,
    lazyFragmentsSet: Set<string>,
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>,
): ts.Node[] {
    const variablesName = clientTypeNameBuilders.variablesTypeName(
        operation.name,
    );
    const resultName = clientTypeNameBuilders.resultTypeName(operation.name);
    return [
        generateOperationZodInputSchema(
            scalarsMapping,
            operation,
            variablesName,
        ),
        generateZodInferInterfaceType(
            'input',
            variablesName,
            generateSchemaName(variablesName),
        ),
        ts.factory.createIdentifier('\n'),
        generateOperationZodOutputSchema(
            scalarsMapping,
            schema,
            lazyFragmentsSet,
            operation,
            resultName,
        ),
        generateZodInferInterfaceType(
            'output',
            resultName,
            generateSchemaName(resultName),
        ),
        generateOperationNode(clientTypeNameBuilders, schema, operation),
        ts.factory.createIdentifier('\n'),
    ];
}

export function generateOperationsNodes(
    clientTypeNameBuilders: ClientTypeNameBuilders,
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
): ts.Node[] {
    return Object.values(schema.client.operations)
        .map((operation) =>
            generateOperationNodes(
                clientTypeNameBuilders,
                scalarsMapping,
                lazyFragmentsSet,
                schema,
                operation,
            ),
        )
        .flat();
}
