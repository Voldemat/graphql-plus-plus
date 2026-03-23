import { RootSchema } from './schema/root.js';

export interface ActorContext {
    readonly schema: RootSchema
}
export type Actor<T extends ActorContext> = (
    c: T,
) => Promise<void> | void
export interface Config<T extends ActorContext> {
    context: T
    actors: Actor<T>[]
}
