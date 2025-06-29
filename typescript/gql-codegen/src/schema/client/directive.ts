import { z } from 'zod/v4';
import { inputFieldSchema } from '../shared.js';

export const directiveSchema = z.object({
    name: z.string(),
    arguments: z.record(z.string(), inputFieldSchema),
    locations: z.array(z.enum([
        'QUERY',
        'MUTATION',
        'SUBSCRIPTION',
        'FIELD',
        'FRAGMENT_DEFINITION',
        'FRAGMENT_SPREAD',
        'INLINE_FRAGMENT',
        'VARIABLE_DEFINITION',
    ]))
})

