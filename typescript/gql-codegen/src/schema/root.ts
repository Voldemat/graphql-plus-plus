import { z } from 'zod/v4'
import { serverSchema } from './server.js'
import { clientSchema } from './client/root.js'

export const rootSchema = z.object({
    server: serverSchema,
    client: clientSchema
}).strict()

export type RootSchema = z.infer<typeof rootSchema>
