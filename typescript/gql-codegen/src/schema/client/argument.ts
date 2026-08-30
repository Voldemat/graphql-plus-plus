import { z } from 'zod/v4';
import { literalSchema } from '../shared.js';

export const argumentValue = z.discriminatedUnion('_type', [
    z.object({
        _type: z.literal('ref'),
        name: z.string(),
    }),
    z.object({
        _type: z.literal('literal'),
        literal: literalSchema,
    }),
]);

export const argument = z.object({
    name: z.string(),
    value: argumentValue,
});
