import ts from 'typescript';
import { ActorContext } from '../../../../config.js';
import { GraphqlActorConfig } from '../../actor.js';
import { generateFragmentTypes } from './fragments.js';
import { addNewLineBetweenNodes } from '../../../ts-shared.js';
import { generateOperationsNodes } from './operations.js';

export function generateClientNodes(
    config: GraphqlActorConfig,
    context: ActorContext,
    scalars: string[]
): ts.Node[] {
    return [
        ...addNewLineBetweenNodes(generateFragmentTypes(
            scalars,
            context.schema
        )),
        ...addNewLineBetweenNodes(generateOperationsNodes(
            scalars,
            context.schema,
        ))
    ]
}
