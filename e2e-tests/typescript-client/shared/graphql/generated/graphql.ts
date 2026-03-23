import { z } from "zod/v4";


export const checkSchema = z.object({
    a: z.number()
});
export type Check = z.output<typeof checkSchema>;

export const querySchema = z.object({
    get getCheck() {
        return checkSchema;
    }
});
export type Query = z.output<typeof querySchema>;

export interface Operation<V, R> {
    name: string;
    type: "QUERY" | "MUTATION" | "SUBSCRIPTION";
    document: string;
    variablesSchema: z.ZodType<unknown, V>;
    resultSchema: z.ZodType<R>;
}
export const CheckFragmentDocument = "fragment Check on Check{__typename a}";
export const checkFragmentSchema = z.object({
    a: z.number(),
    __typename: z.literal("Check").nullable().optional()
});
export type CheckFragment = z.output<typeof checkFragmentSchema>;
export const getCheckVariablesSchema = z.object({});
export type GetCheckVariables = z.input<typeof getCheckVariablesSchema>;

export const getCheckResultSchema = z.object({
    __typename: z.literal("Query").nullable().optional(),
    getCheck: z.lazy(() => z.object({
        ...checkFragmentSchema.shape,
        __typename: z.literal("Check").nullable().optional()
    }))
});
export type GetCheckResult = z.output<typeof getCheckResultSchema>;
export const GetCheckOperation = {
    name: "GetCheck",
    type: "QUERY",
    document: "query GetCheck{getCheck{...Check}} fragment Check on Check{__typename a}",
    variablesSchema: getCheckVariablesSchema,
    resultSchema: getCheckResultSchema
} as const satisfies Operation<GetCheckVariables, GetCheckResult>;

