import { Actor, ActorContext } from '@/config.js';
import { PathOrFileDescriptor, writeFileSync } from 'fs';
import ts from 'typescript';
import { renderNodes, TSActorConfig } from '../shared.js';
import { generateNodes } from './generators/main.js';
import { ScalarsMapping } from './generators/server/scalars/index.js';

export interface Config extends TSActorConfig {
    outPath: PathOrFileDescriptor;
    scalarsMapping: ScalarsMapping;
    importDeclarations: ts.ImportDeclaration[];
    onlyRequiredForOperations: boolean;
}

async function actor(config: Config, context: ActorContext) {
    const nodes = generateNodes(config, context);
    const code = await renderNodes(config, nodes);
    writeFileSync(config.outPath, code);
}

export function build(config: Config): Actor<ActorContext> {
    return (context) => actor(config, context);
}
