import { ClientSchema } from './client/root.js';
import { ServerSchema } from './server.js';

export interface RootSchema {
    server: ServerSchema
    client: ClientSchema
}
