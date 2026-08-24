import { RootSchema } from './schema/root.js';

export interface ActorContext {
    readonly schema: RootSchema;
}

export enum RunAction {
    Generate,
    Validate,
}

export function runActionFromArgv(argv: string[]): RunAction {
    if (argv.length < 3) {
        throw new Error(
            'argv length is less than 3, action is not provided, valid actions are: "generate" and "validate"',
        );
    }
    switch (argv[2]) {
        case 'validate':
            return RunAction.Validate;
        case 'generate':
            return RunAction.Generate;
        default:
            throw new Error(`Unknown action: ${argv[2]}`);
    }
}

export type Actor<T extends ActorContext> = (
    c: T,
    action: RunAction,
) => Promise<void> | void;
export interface Config<T extends ActorContext> {
    context: T;
    actors: Actor<T>[];
}
