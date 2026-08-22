import { ActorContext } from '@/config.js';
import ts from 'typescript';
import { Config } from '../../actor.js';
import { generateFragmentTypes } from './fragments.js';
import { generateOperationsNodes } from './operations.js';

export function generateClientNodes(
    config: Config,
    context: ActorContext,
): ts.Node[] {
    return [
        ...generateFragmentTypes(config.scalarsMapping, context.schema),
        ...generateOperationsNodes(
            config.clientTypeNameBuilders,
            config.scalarsMapping,
            context.schema,
        ),
    ];
}
