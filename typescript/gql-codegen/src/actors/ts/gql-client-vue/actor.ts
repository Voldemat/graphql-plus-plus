import { Actor, ActorContext } from '@/config.js';
import { PathOrFileDescriptor, writeFileSync } from 'fs';
import ts from 'typescript';
import { renderNodes, TSActorConfig } from '../shared.js';
import { generateNodes } from './generators/main.js';

export interface Config extends TSActorConfig {
    outPath: PathOrFileDescriptor;
    importDeclarations: ts.ImportDeclaration[];
    graphqlModulePath: string;
    sdk: {
        queriesKey: string;
        mutationsKey: string;
        subscriptionsKey: string;
    };
}

async function actor(config: Config, context: ActorContext) {
    const nodes = generateNodes(config, context);
    const code = await renderNodes(config, nodes);
    writeFileSync(config.outPath, code);
}

export function build(config: Config): Actor<ActorContext> {
    return (context) => actor(config, context);
}
