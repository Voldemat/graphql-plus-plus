import ts from 'typescript';
import { GraphqlActorConfig } from '../../actor.js';
import { ActorContext } from '@/config.js';
import { generateEnumDefinition } from './enums.js';
import { generateUnionTypeDefinitions } from './unions.js';
import { generateInputTypeDefinitions } from './inputs.js';
import { addNewLineBetweenNodes } from '../../../shared.js';
import { generateObjectTypeNodes } from './objects.js';

export function generateServerNodes(
    config: GraphqlActorConfig,
    context: ActorContext,
): ts.Node[] {
    return [
        ...addNewLineBetweenNodes(
            Object.values(context.schema.server.enums)
                .map(generateEnumDefinition)
        ),
        ts.factory.createIdentifier('\n'),
        ...Object.values(context.schema.server.objects)
            .map(object =>
                generateObjectTypeNodes(config.scalarsMapping, object)).flat(),
        ...Object.values(context.schema.server.unions)
            .map(union =>
                generateUnionTypeDefinitions(
                    config.scalarsMapping,
                    context.schema.server.objects,
                    union
                )).flat(),
        ...Object.values(context.schema.server.inputs)
            .map(input => generateInputTypeDefinitions(
                config.scalarsMapping, input
            ))
            .flat()
    ]
}
