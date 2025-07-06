import ts from 'typescript';
import { ActorContext } from '@/config.js';
import { GraphqlActorConfig } from '../../actor.js';
import { generateFragmentTypes } from './fragments.js';
import { generateOperationsNodes } from './operations.js';

export function generateClientNodes(
    config: GraphqlActorConfig,
    context: ActorContext,
    scalars: string[]
): ts.Node[] {
    return [
        ...generateFragmentTypes(
            scalars,
            context.schema
        ),
        ...generateOperationsNodes(
            config.scalarsMapping,
            context.schema,
        )
    ]
}
