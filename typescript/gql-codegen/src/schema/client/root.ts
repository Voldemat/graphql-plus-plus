import { z } from 'zod/v4';
import { fragmentSchema } from './fragment.js';
import { operationSchema } from './operation.js';
import { directiveSchema } from './directive.js';

export const clientSchema = z.object({
    fragments: z.record(z.string(), fragmentSchema),
    operations: z.record(z.string(), operationSchema),
    directives: z.record(z.string(), directiveSchema)
})
export type ClientSchema = z.infer<typeof clientSchema>;
