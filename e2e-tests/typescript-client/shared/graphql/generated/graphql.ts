/* oxlint-disable no-use-before-define,max-lines */
import { z } from "zod/v4";

export const querySchema = z.object({
    get getUser() {
        return userSchema;
    },
});
export interface Query extends z.output<typeof querySchema> {}

export const subscriptionSchema = z.object({
    get streamUsers() {
        return userSchema;
    },
});
export interface Subscription extends z.output<typeof subscriptionSchema> {}

export const userSchema = z.object({
    email: z.string(),
    id: z.string(),
    name: z.string(),
});
export interface User extends z.output<typeof userSchema> {}

export const BaseUserFragmentDocument =
    "fragment BaseUser on User {\n    __typename\n    id\n}";
export const baseUserFragmentSchema = z.object({
    id: z.string(),
    __typename: z.literal("User").nullable().optional(),
});
export interface BaseUserFragment extends z.output<
    typeof baseUserFragmentSchema
> {}
export const UserFragmentDocument =
    "fragment User on User {\n    ...BaseUser\n    email\n    name\n} fragment BaseUser on User {\n    __typename\n    id\n}";
export const userFragmentSchema = z.object({
    email: z.string(),
    name: z.string(),
    ...z.lazy(() => baseUserFragmentSchema).def.getter().shape,
    __typename: z.literal("User").nullable().optional(),
});
export interface UserFragment extends z.output<typeof userFragmentSchema> {}
export const getUserVariablesSchema = z.object({
    id: z.string(),
});
export interface GetUserVariables extends z.input<
    typeof getUserVariablesSchema
> {}

export const getUserResultSchema = z.object({
    __typename: z.literal("Query").nullable().optional(),
    getUser: z.object({
        ...userFragmentSchema.shape,
        __typename: z.literal("User").nullable().optional(),
    }),
});
export interface GetUserResult extends z.output<typeof getUserResultSchema> {}
export const GetUserOperation = {
    name: "GetUser",
    type: "QUERY",
    document:
        "query GetUser($id: UUID!) {\n    getUser(id: $id) {\n        ...User\n    }\n} fragment User on User {\n    ...BaseUser\n    email\n    name\n} fragment BaseUser on User {\n    __typename\n    id\n}",
    variablesSchema: getUserVariablesSchema,
    resultSchema: getUserResultSchema,
} as const;

export const streamUsersVariablesSchema = z.object({});
export interface StreamUsersVariables extends z.input<
    typeof streamUsersVariablesSchema
> {}

export const streamUsersResultSchema = z.object({
    __typename: z.literal("Subscription").nullable().optional(),
    streamUsers: z.object({
        ...userFragmentSchema.shape,
        __typename: z.literal("User").nullable().optional(),
    }),
});
export interface StreamUsersResult extends z.output<
    typeof streamUsersResultSchema
> {}
export const StreamUsersOperation = {
    name: "StreamUsers",
    type: "SUBSCRIPTION",
    document:
        "subscription StreamUsers {\n    streamUsers {\n        ...User\n    }\n} fragment User on User {\n    ...BaseUser\n    email\n    name\n} fragment BaseUser on User {\n    __typename\n    id\n}",
    variablesSchema: streamUsersVariablesSchema,
    resultSchema: streamUsersResultSchema,
} as const;
