import path from 'path';
import fs from 'fs';
import pc from 'picocolors';
import { RunAction } from '../config.js';
import { diffLines } from 'diff';
import { printChanges } from './text-diff.js';

export function executeRunAction(
    outPath: fs.PathOrFileDescriptor,
    action: RunAction,
    code: string,
) {
    switch (action) {
        case RunAction.Generate: {
            fs.writeFileSync(outPath, code);
            break;
        }
        case RunAction.Validate: {
            const fileCode = fs.readFileSync(outPath).toString();
            if (fileCode !== code) {
                const changes = diffLines(fileCode, code);
                process.stdout.write(
                    pc.blue(
                        `${path.relative(process.cwd(), outPath.toString())}:\n`,
                    ),
                );
                printChanges(changes);
            }
        }
    }
}
