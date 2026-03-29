// @ts-nocheck
import type { IExecutor, RequestContext, SubOpAsyncIterable } from "@vladimirdev635/gql-client/types";
import { GetUserOperation, GetUserVariables, GetUserResult, StreamUsersOperation, StreamUsersVariables, StreamUsersResult } from "./graphql.ts";

export function createSdk<TRequestContext extends RequestContext>(executor: IExecutor<TRequestContext>) {
    return {
        queries: {
            GetUser: async (variables: GetUserVariables, requestContext: TRequestContext): Promise<GetUserResult> => {
                const executorResult = await executor.executeSync(GetUserOperation, variables, requestContext);
                return executorResult.result;
            }
        },
        subscriptions: {
            StreamUsers: async (variables: StreamUsersVariables, requestContext: TRequestContext, controller: AbortController): Promise<SubOpAsyncIterable<StreamUsersResult>> => {
                const executorResult = await executor.executeSubscription(StreamUsersOperation, variables, requestContext, controller);
                return executorResult.result;
            }
        }
    } as const;
}
