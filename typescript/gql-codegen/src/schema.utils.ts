import { PathOrFileDescriptor, readFileSync } from "fs";
import { rootSchema, RootSchema } from "./schema.js";

export function loadSchemaFromFile(p: PathOrFileDescriptor): RootSchema {
    return rootSchema.parse(JSON.parse(readFileSync(p).toString()))
}
