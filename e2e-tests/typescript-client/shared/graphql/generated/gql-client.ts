// @ts-nocheck
import type { IExecutor, RequestContext } from "@vladimirdev635/gql-client/types";
import { GetCheckOperation, GetCheckVariables, GetCheckResult } from "./graphql.ts";

export function createSdk<TRequestContext extends RequestContext>(executor: IExecutor<TRequestContext>) {
    return {
        queries: {
            GetCheck: async (variables: GetCheckVariables, requestContext: TRequestContext): Promise<GetCheckResult> => {
                const executorResult = await executor.executeSync(GetCheckOperation, variables, requestContext);
                return executorResult.result;
            }
        }
    } as const;
}
