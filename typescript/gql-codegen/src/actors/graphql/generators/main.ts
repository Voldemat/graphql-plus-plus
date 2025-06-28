import ts from 'typescript';
import { GraphqlActorConfig } from '../actor.js';
import { generateScalarsInterfaceDefinition } from './scalars/generators.js';
import { ActorContext } from '../../../config.js';
import { generateEnumDefinition } from './enums.js';
import { generateObjectInterfaceDefinition } from './objects.js';
import { generateUnionTypeDefinition } from './unions.js';
import { generateInputTypeDefinition } from './inputs.js';
import { generateQueriesVariablesDefinitions } from './variables.js';
import { addNewLineBetweenNodes } from '../../ts-shared.js';
import { helperNodes } from './helperNodes.js';

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
        generateScalarsInterfaceDefinition(
            config.scalarsMapping,
            context.schema.server.scalars
        ),
        ts.factory.createIdentifier('\n'),
        ...addNewLineBetweenNodes(
            Object.values(context.schema.server.enums)
                .map(generateEnumDefinition)
        ),
        ts.factory.createIdentifier('\n'),
        ...addNewLineBetweenNodes(
            Object.values(context.schema.server.objects)
                .map(object =>
                    generateObjectInterfaceDefinition(scalars, object))
        ),
        ...Object.values(context.schema.server.unions)
            .map(union => generateUnionTypeDefinition(scalars, union)),
        ...addNewLineBetweenNodes(
            Object.values(context.schema.server.inputs)
                .map(input => generateInputTypeDefinition(scalars, input))
        ),
        ...addNewLineBetweenNodes(
            Object.entries(context.schema.server.objects['Query'].fields)
                .map(([name, field]) =>
                    generateQueriesVariablesDefinitions(scalars, name, field))
        )
    ]
}
