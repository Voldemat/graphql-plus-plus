import { PathOrFileDescriptor, readFileSync } from 'fs';
import { ServerSchema, serverSchema } from './server.js';
import { ClientSchema, clientSchema } from './client/root.js';

export function loadServerSchemaFromFile(
    p: PathOrFileDescriptor
): ServerSchema {
    return serverSchema.parse(JSON.parse(readFileSync(p).toString()))
}

export function loadClientSchemaFromFile(
    p: PathOrFileDescriptor
): ClientSchema {
    return clientSchema.parse(JSON.parse(readFileSync(p).toString()))
}
