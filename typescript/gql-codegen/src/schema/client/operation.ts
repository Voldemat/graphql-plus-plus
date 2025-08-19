import { z } from 'zod/v4';
import { fragmentSpecSchema } from './fragment.js';
import { inputFieldSchema } from '../shared.js';

export const operationSchema = z.object({
    name: z.string(),
    type: z.enum(['MUTATION', 'QUERY', 'SUBSCRIPTION']),
    parameters: z.record(z.string(), inputFieldSchema),
    fragmentSpec: fragmentSpecSchema,
    sourceText: z.string(),
    hash: z.string()
})

