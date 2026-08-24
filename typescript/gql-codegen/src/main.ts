import { ActorContext, Config, RunAction } from './config.js';

export async function run<TContext extends ActorContext>(
    config: Config<TContext>,
    action: RunAction,
) {
    for (const actor of config.actors) {
        await actor(config.context, action);
    }
}
