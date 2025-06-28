import { RootSchema } from './schema.js';
import { Logger } from 'pino';

export interface ActorContext {
    readonly schema: RootSchema
    readonly logger: Logger
}
export type Actor<T extends ActorContext> = (
    c: T,
) => Promise<void> | void
export interface Config<T extends ActorContext> {
    context: T
    actors: Actor<T>[]
}
