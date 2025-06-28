import { ActorContext, Config } from './config.js';

export async function run<TContext extends ActorContext>(
    config: Config<TContext>
) {
    for (const actor of config.actors) {
        await actor(config.context)
    }
}
