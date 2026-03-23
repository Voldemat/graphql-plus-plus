import {
    createSerializer,
    createJSONSerializer,
    createMultipartSerializer
} from '@vladimirdev635/gql-client/serializers'
import { Executor } from '@vladimirdev635/gql-client'
import {
    type IExecutor,
    type RequestContext
} from '@vladimirdev635/gql-client/types/index'
import { createParser } from '@vladimirdev635/gql-client/parsers'

export interface Context {
    accessToken: string | null
    refreshToken: string | null
}

export function createExecutor(
    initContext: Context
): IExecutor<RequestContext> {
    return new Executor<Context, RequestContext>({
        context: initContext,
        retryConfig: {
            shouldSyncRetry: () => false,
            shouldSubscriptionRetry: () => false
        },
        middlewares: {
            beforeSerialization: [],
            afterSerialization: {
                sync: [],
                subscription: []
            },
            beforeParsing: {
                sync: [],
                subscription: []
            },
            afterParsing: {
                sync: [],
                subscription: []
            }
        },
        fetcher: init => fetch(import.meta.env.ENDPOINT_URL!, init),
        serializer: createSerializer(
            createJSONSerializer(),
            createMultipartSerializer()
        ),
        parser: createParser()
    })
}
