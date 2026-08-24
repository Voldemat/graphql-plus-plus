import { PathOrFileDescriptor } from 'fs';
import ts from 'typescript';
import { Actor, ActorContext, RunAction } from '../../../config.js';
import {
    ClientTypeNameBuilders,
    renderNodes,
    TSActorConfig,
} from '../shared.js';
import { executeRunAction } from '../../utils.js';
import { generateNodes } from './generators/main.js';
import { ScalarsMapping } from './generators/server/scalars/index.js';

export interface Config extends TSActorConfig {
    outPath: PathOrFileDescriptor;
    scalarsMapping: ScalarsMapping;
    importDeclarations: ts.ImportDeclaration[];
    onlyRequiredForOperations: boolean;
    clientTypeNameBuilders: ClientTypeNameBuilders;
}

async function actor(config: Config, context: ActorContext, action: RunAction) {
    const nodes = generateNodes(config, context);
    const code = await renderNodes(config, nodes);
    executeRunAction(config.outPath, action, code);
}

export function build(config: Config): Actor<ActorContext> {
    return (context, action) => actor(config, context, action);
}
