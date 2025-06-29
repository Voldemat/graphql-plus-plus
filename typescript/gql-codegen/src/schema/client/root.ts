import { z } from 'zod/v4';
import { fragmentSpecSchema } from './fragment.js';
import { operationSchema } from './operation.js';
import { directiveSchema } from './directive.js';

export const clientSchema = z.object({
    fragments: z.record(z.string(), fragmentSpecSchema),
    operations: z.record(z.string(), operationSchema),
    directives: z.record(z.string(), directiveSchema)
})
