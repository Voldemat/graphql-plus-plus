import { diffLines } from 'diff';
import { PathOrFileDescriptor, readFileSync, writeFileSync } from 'fs';
import path from 'path';
import pc from 'picocolors';
import ts from 'typescript';
import { Actor, ActorContext, RunAction } from '../../../config.js';
import {
    ClientTypeNameBuilders,
    renderNodes,
    TSActorConfig,
} from '../shared.js';
import { printChanges } from '../text-diff.js';
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
    switch (action) {
        case RunAction.Generate: {
            writeFileSync(config.outPath, code);
            break;
        }
        case RunAction.Validate: {
            const fileCode = readFileSync(config.outPath).toString();
            const changes = diffLines(fileCode, code);
            process.stdout.write(
                pc.blue(
                    `${path.relative(process.cwd(), config.outPath.toString())}:\n`,
                ),
            );
            printChanges(changes);
        }
    }
}

export function build(config: Config): Actor<ActorContext> {
    return (context, action) => actor(config, context, action);
}
