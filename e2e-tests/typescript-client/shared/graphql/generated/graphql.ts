import { z } from "zod/v4";


export const querySchema = z.object({
    get getUser() {
        return userSchema;
    }
});
export type Query = z.output<typeof querySchema>;

export const subscriptionSchema = z.object({
    get streamUsers() {
        return userSchema;
    }
});
export type Subscription = z.output<typeof subscriptionSchema>;

export const userSchema = z.object({
    email: z.string(),
    id: z.string(),
    name: z.string()
});
export type User = z.output<typeof userSchema>;

export interface Operation<V, R> {
    name: string;
    type: "QUERY" | "MUTATION" | "SUBSCRIPTION";
    document: string;
    variablesSchema: z.ZodType<unknown, V>;
    resultSchema: z.ZodType<R>;
}
export const UserFragmentDocument = "fragment User on User{__typename email id name}";
export const userFragmentSchema = z.object({
    email: z.string(),
    id: z.string(),
    name: z.string(),
    __typename: z.literal("User").nullable().optional()
});
export type UserFragment = z.output<typeof userFragmentSchema>;
export const getUserVariablesSchema = z.object({
    id: z.string()
});
export type GetUserVariables = z.input<typeof getUserVariablesSchema>;

export const getUserResultSchema = z.object({
    __typename: z.literal("Query").nullable().optional(),
    getUser: z.lazy(() => z.object({
        ...userFragmentSchema.shape,
        __typename: z.literal("User").nullable().optional()
    }))
});
export type GetUserResult = z.output<typeof getUserResultSchema>;
export const GetUserOperation = {
    name: "GetUser",
    type: "QUERY",
    document: "query GetUser($id:UUID!){getUser(id:$id){...User}} fragment User on User{__typename email id name}",
    variablesSchema: getUserVariablesSchema,
    resultSchema: getUserResultSchema
} as const satisfies Operation<GetUserVariables, GetUserResult>;

export const streamUsersVariablesSchema = z.object({});
export type StreamUsersVariables = z.input<typeof streamUsersVariablesSchema>;

export const streamUsersResultSchema = z.object({
    __typename: z.literal("Subscription").nullable().optional(),
    streamUsers: z.lazy(() => z.object({
        ...userFragmentSchema.shape,
        __typename: z.literal("User").nullable().optional()
    }))
});
export type StreamUsersResult = z.output<typeof streamUsersResultSchema>;
export const StreamUsersOperation = {
    name: "StreamUsers",
    type: "SUBSCRIPTION",
    document: "subscription StreamUsers{streamUsers{...User}} fragment User on User{__typename email id name}",
    variablesSchema: streamUsersVariablesSchema,
    resultSchema: streamUsersResultSchema
} as const satisfies Operation<StreamUsersVariables, StreamUsersResult>;

