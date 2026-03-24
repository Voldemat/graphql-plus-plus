// @ts-nocheck
import type { IExecutor, RequestContext } from "@vladimirdev635/gql-client/types";
import { GetUserOperation, GetUserVariables, GetUserResult } from "./graphql.ts";

export function createSdk<TRequestContext extends RequestContext>(executor: IExecutor<TRequestContext>) {
    return {
        queries: {
            GetUser: async (variables: GetUserVariables, requestContext: TRequestContext): Promise<GetUserResult> => {
                const executorResult = await executor.executeSync(GetUserOperation, variables, requestContext);
                return executorResult.result;
            }
        }
    } as const;
}
