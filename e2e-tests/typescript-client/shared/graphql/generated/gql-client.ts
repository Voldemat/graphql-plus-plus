/* oxlint-disable no-use-before-define,max-lines */
import type { types } from "@vladimirdev635/gql-client";
import {
    GetUserOperation,
    type GetUserVariables,
    type GetUserResult,
    StreamUsersOperation,
    type StreamUsersVariables,
    type StreamUsersResult,
} from "./graphql.ts";

type GQLSyncMethodFuncType<TRequestContext, V, R> = (
    variables: V,
    context: TRequestContext,
) => Promise<R>;
type GQLSubscriptionMethodFuncType<TRequestContext, V, R> = (
    variables: V,
    context: TRequestContext,
    controller: AbortController,
) => Promise<types.SubOpAsyncIterable<R>>;
export interface GQLQueryRequests<TRequestContext> {
    GetUser: GQLSyncMethodFuncType<
        TRequestContext,
        GetUserVariables,
        GetUserResult
    >;
}
export interface GQLSubscriptionRequests<TRequestContext> {
    StreamUsers: GQLSubscriptionMethodFuncType<
        TRequestContext,
        StreamUsersVariables,
        StreamUsersResult
    >;
}
export interface SdkType<TRequestContext> {
    queries: GQLQueryRequests<TRequestContext>;
    subscriptions: GQLSubscriptionRequests<TRequestContext>;
}
function buildSyncResultCallback<
    TExecutor extends types.IExecutor<TRequestContext>,
    TRequestContext extends types.RequestContext,
    V,
    R,
>(
    executor: TExecutor,
    operation: types.SyncOperation<V, R>,
): GQLSyncMethodFuncType<TRequestContext, V, R> {
    return async (
        variables: V,
        requestContext: TRequestContext,
    ): Promise<R> => {
        const executorResult = await executor.executeSync(
            operation,
            variables,
            requestContext,
        );
        return executorResult.result;
    };
}
function buildSubscriptionResultCallback<
    TExecutor extends types.IExecutor<TRequestContext>,
    TRequestContext extends types.RequestContext,
    V,
    R,
>(
    executor: TExecutor,
    operation: types.SubscriptionOperation<V, R>,
): GQLSubscriptionMethodFuncType<TRequestContext, V, R> {
    return async (
        variables: V,
        requestContext: TRequestContext,
        controller: AbortController,
    ): Promise<types.SubOpAsyncIterable<R>> => {
        const executorResult = await executor.executeSubscription(
            operation,
            variables,
            requestContext,
            controller,
        );
        return executorResult.result;
    };
}

export function createSdk<
    TExecutor extends types.IExecutor<TRequestContext>,
    TRequestContext extends types.RequestContext,
>(executor: TExecutor): SdkType<TRequestContext> {
    return {
        queries: {
            GetUser: buildSyncResultCallback<
                TExecutor,
                TRequestContext,
                GetUserVariables,
                GetUserResult
            >(executor, GetUserOperation),
        },
        subscriptions: {
            StreamUsers: buildSubscriptionResultCallback<
                TExecutor,
                TRequestContext,
                StreamUsersVariables,
                StreamUsersResult
            >(executor, StreamUsersOperation),
        },
    } as const;
}
