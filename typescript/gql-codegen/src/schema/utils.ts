import { PathOrFileDescriptor, readFileSync } from 'fs';
import { rootSchema, RootSchema } from './root.js';

export function loadSchemaFromFile(p: PathOrFileDescriptor): RootSchema {
    return rootSchema.parse(JSON.parse(readFileSync(p).toString()))
}
